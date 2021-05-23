use super::*;
use indicatif::ParallelProgressIterator;
use num_traits::Pow;
use rayon::prelude::*;
use std::convert::TryFrom;

pub enum Distance {
    L2,
    Cosine,
}

impl TryFrom<&str> for Distance {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "L2" => Ok(Distance::L2),
            "COSINE" =>  Ok(Distance::Cosine),
            _ => Err(format!("Unknown distance metric {}", value))
        }
    }
}

/// # Methods to thicken the graph.
impl Graph {
    /// Returns graph with edges added extracted from given node_features.
    ///
    /// # Arguments
    /// * `features`: Vec<Vec<f64>> - node_features to use to identify the new neighbours.
    /// * `neighbours_number`: Option<NodeT> - Number of neighbours to add.
    /// * `distance_name`: Option<&str> - Name of distance to use. Can either be L2 or COSINE. By default COSINE.
    /// * `verbose`: Option<bool> - Whether to show loading bars.
    ///
    /// # Raises
    /// * If the graph does not have nodes.
    /// * If the given node_features are not provided exactly for each node.
    /// * If the node_features do not have a consistent shape.
    /// * If the provided number of neighbours is zero.
    pub fn generate_new_edges_from_node_features(
        &self,
        features: Vec<Vec<f64>>,
        neighbours_number: Option<NodeT>,
        distance_name: Option<&str>,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        // check that the parameters are sane
        self.must_have_nodes()?;
        if features.len() != self.get_nodes_number() as usize {
            return Err(format!(
                concat!(
                    "The node features length need to be provided for each of the node, ",
                    "but the provided node features length is {} while the number of ",
                    "nodes in the graph is {}."
                ),
                features.len(),
                self.get_nodes_number()
            ));
        }
        let expected_node_features_length = features.first().unwrap().len();
        for node_features in features.iter() {
            if expected_node_features_length != node_features.len() {
                return Err(format!(
                    concat!(
                        "The node features length needs to be consistent: the expected ",
                        "size was {} while the found length was {}."
                    ),
                    expected_node_features_length,
                    node_features.len()
                ));
            }
        }

        // compute the neighbours nodes to add
        let neighbours_number =
            neighbours_number.unwrap_or(self.get_node_degrees_mean()?.ceil() as NodeT);
        if neighbours_number == 0 {
            return Err("The number of neighbours to add per node cannot be zero!".to_string());
        }

        // initialize the progress bar
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing additional edges to thicken graph",
            self.get_nodes_number() as usize,
        );

        // initialize the distance metric
        let distance_metric = match Distance::try_from(distance_name.unwrap_or("COSINE"))? {
            Distance::L2 => {
                |current_node_features: &Vec<f64>, node_features: &Vec<f64>| -> f64 {
                    current_node_features
                    .iter()
                    .zip(node_features.iter())
                    .map(|(&left, &right)| (left - right).pow(2))
                    .sum()
                }
            }
            Distance::Cosine => {
                |current_node_features: &Vec<f64>, node_features: &Vec<f64>| -> f64 {
                    let numerator = current_node_features
                        .iter()
                        .zip(node_features.iter())
                        .map(|(&left, &right)| left * right)
                        .sum::<f64>();
                    let denominator_left = current_node_features
                        .iter()
                        .map(|&left| left.pow(2))
                        .sum::<f64>()
                        .sqrt();
                    let denominator_right = node_features
                        .iter()
                        .map(|&right| right.pow(2))
                        .sum::<f64>()
                        .sqrt();
                    1.0 - numerator / (denominator_left * denominator_right + f64::EPSILON)
                }
            }
        };

        // compute the new edges to add
        let new_edges = self
            .par_iter_node_ids()
            .progress_with(pb)
            .map(|source_node_id| {
                // for each node find the k closest nodes (based on the distance choosen and their features)
                let current_node_features = &features[source_node_id as usize];
                let mut closest_nodes_distances = vec![f64::INFINITY; neighbours_number as usize];
                let mut closest_nodes = Vec::with_capacity(neighbours_number as usize);

                features.iter().zip(self.iter_node_ids())
                // every node is the closest to itself so we filter it out
                .filter( |(_, destination_node_id)| source_node_id != *destination_node_id)
                .for_each(
                    |(node_features, destination_node_id)| {
                        // compute the distance
                        let distance = distance_metric(current_node_features, node_features);
                        // get the max distance in the currently cosest nodes
                        let (i, max_distance) = unsafe {
                            closest_nodes_distances.argmax().unwrap_unchecked()
                        };
                        // update the closest nodes inserting the current node if needed
                        if max_distance > distance {
                            if max_distance == f64::INFINITY {
                                closest_nodes.push(destination_node_id);
                            } else {
                                closest_nodes[i] = destination_node_id;
                            }
                            closest_nodes_distances[i] = distance;
                        }
                    },
                );

                closest_nodes
            })
            .collect::<Vec<Vec<NodeT>>>();

        
        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id_and_edge_weight(true)
                .map(|(_, src_node_id, dst_node_id, edge_type, weight)| {
                    Ok((src_node_id, dst_node_id, edge_type, weight))
                })
                .chain(
                    new_edges
                        .into_iter()
                        .enumerate()
                        .map(|(source_node_id, new_neighbours)| {
                            new_neighbours
                                .into_iter()
                                .map(move |destination_node_id| {
                                    if !self.is_directed() {
                                        vec![
                                            Ok((source_node_id as NodeT, destination_node_id, None, None)),
                                            Ok((destination_node_id, source_node_id as NodeT, None, None)),
                                        ]
                                    } else {
                                        vec![Ok((source_node_id as NodeT, destination_node_id, None, None))]
                                    }
                                })
                                .flatten()
                        })
                        .flatten(),
                ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose,
        )
    }
}