use super::*;
use indicatif::ProgressIterator;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use roaring::{RoaringBitmap, RoaringTreemap};
use std::collections::HashSet;
use std::iter::FromIterator;
use vec_rand::gen_random_vec;
use vec_rand::xorshift::xorshift as rand_u64;

/// # Holdouts.
impl Graph {
    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// skipgrams algorithm.
    ///
    ///
    /// # Arguments
    ///
    /// * `random_state`: EdgeT - random_state to use to reproduce negative edge set.
    /// * `negatives_number`: EdgeT - Number of negatives edges to include.
    /// * `seed_graph`: Option<Graph> - Optional graph to use to filter the negative edges. The negative edges generated when this variable is provided will always have a node within this graph.
    /// * `verbose`: bool - Wether to show the loading bar.
    ///
    pub fn sample_negatives(
        &self,
        mut random_state: EdgeT,
        negatives_number: EdgeT,
        seed_graph: Option<&Graph>,
        verbose: bool,
    ) -> Result<Graph, String> {
        if negatives_number == 0 {
            return Err(String::from("The number of negatives cannot be zero."));
        }
        let seed_nodes: Option<RoaringBitmap> = if let Some(sg) = &seed_graph {
            if !self.overlaps(&sg)? {
                return Err(String::from(
                    "The given seed graph does not overlap with the current graph instance.",
                ));
            }
            Some(RoaringBitmap::from_iter(sg.get_nodes_names_iter().map(
                |(node_name, _)| self.get_unchecked_node_id(&node_name),
            )))
        } else {
            None
        };
        // In a complete directed graph allowing selfloops with N nodes there are N^2
        // edges. In a complete directed graph without selfloops there are N*(N-1) edges.
        // We can rewrite the first formula as (N*(N-1)) + N.
        //
        // In a complete undirected graph allowing selfloops with N nodes there are
        // (N*(N-1))/2 + N edges.

        // Here we use unique edges number because on a multigraph the negative
        // edges cannot have an edge type.
        let nodes_number = self.get_nodes_number() as EdgeT;

        // Here we compute the number of edges that a complete graph would have if it had the same number of nodes
        // of the current graph. Moreover, the complete graph will have selfloops IFF the current graph has at
        // least one of them.
        let mut complete_edges_number: EdgeT = nodes_number * (nodes_number - 1);
        if self.has_selfloops() {
            complete_edges_number += nodes_number;
        }
        // Now we compute the maximum number of negative edges that we can actually generate
        let max_negative_edges = complete_edges_number - self.unique_edges_number;

        // We check that the number of requested negative edges is compatible with the
        // current graph instance.
        if negatives_number > max_negative_edges {
            return Err(format!(
                concat!(
                    "The requested negatives number {} is more than the ",
                    "number of negative edges that exist in the graph ({})."
                ),
                negatives_number, max_negative_edges
            ));
        }

        let pb1 = get_loading_bar(
            verbose,
            "Computing negative edges",
            negatives_number as usize,
        );

        let pb2 = get_loading_bar(
            verbose,
            "Building negative graph",
            negatives_number as usize,
        );

        // xorshift breaks if the random_state is zero
        // so we initialize xor it with a constat
        // to mitigate this problem
        random_state ^= SEED_XOR as EdgeT;

        let mut negative_edges_bitmap = RoaringTreemap::new();
        let chunk_size = max!(4096, negatives_number / 100);
        let mut last_length = 0;

        // randomly extract negative edges until we have the choosen number
        while negative_edges_bitmap.len() < negatives_number {
            // generate two random_states for reproducibility porpouses
            random_state = rand_u64(random_state);

            let edges_to_sample: usize = min!(
                chunk_size,
                match self.is_directed() {
                    true => negatives_number - negative_edges_bitmap.len(),
                    false => ((negatives_number - negative_edges_bitmap.len()) as f64 / 2.0).ceil()
                        as u64,
                }
            ) as usize;

            // generate the random edge-sources
            negative_edges_bitmap.extend(
                gen_random_vec(edges_to_sample, random_state)
                    .into_par_iter()
                    // convert them to plain (src, dst)
                    .filter_map(|edge| {
                        let (mut src, mut dst) = self.decode_edge(edge);
                        src %= nodes_number as NodeT;
                        dst %= nodes_number as NodeT;
                        if let Some(sn) = &seed_nodes {
                            if !sn.contains(src) && !sn.contains(dst) {
                                return None;
                            }
                        }
                        // If the edge is not a self-loop or the user allows self-loops and
                        // the graph is directed or the edges are inserted in a way to avoid
                        // inserting bidirectional edges.
                        match (self.has_selfloops() || src != dst) && !self.has_edge(src, dst, None)
                        {
                            true => Some((src, dst)),
                            false => None,
                        }
                    })
                    .flat_map(|(src, dst)| {
                        if !self.is_directed() && src != dst {
                            vec![self.encode_edge(src, dst), self.encode_edge(dst, src)]
                        } else {
                            vec![self.encode_edge(src, dst)]
                        }
                    })
                    .collect::<Vec<EdgeT>>(),
            );

            pb1.inc(negative_edges_bitmap.len() - last_length);
            last_length = negative_edges_bitmap.len();
        }

        pb1.finish();

        Graph::build_graph(
            negative_edges_bitmap.iter().progress_with(pb2).map(|edge| {
                let (src, dst) = self.decode_edge(edge);
                Ok((src, dst, None, None))
            }),
            negative_edges_bitmap.len(),
            self.nodes.clone(),
            self.node_types.clone(),
            None,
            self.directed,
            format!("{} negatives", self.name.clone()),
            false,
        )
    }

    /// Compute the training and validation edges number from the training rate
    fn get_holdouts_edges_number(
        &self,
        train_size: f64,
        include_all_edge_types: bool,
    ) -> Result<(EdgeT, EdgeT), String> {
        if train_size <= 0.0 || train_size >= 1.0 {
            return Err(String::from("Train rate must be strictly between 0 and 1."));
        }
        if self.directed && self.get_edges_number() == 1
            || !self.directed && self.get_edges_number() == 2
        {
            return Err(String::from(
                "The current graph instance has only one edge. You cannot build an holdout with one edge.",
            ));
        }
        let total_edges_number = if include_all_edge_types {
            self.unique_edges_number
        } else {
            self.get_edges_number()
        };
        let train_edges_number = (total_edges_number as f64 * train_size) as EdgeT;
        let valid_edges_number = total_edges_number - train_edges_number;

        if train_edges_number == 0 || train_edges_number >= total_edges_number {
            return Err(String::from(
                "The training set has 0 edges! Change the training rate.",
            ));
        }
        if valid_edges_number == 0 {
            return Err(String::from(
                "The validation set has 0 edges! Change the training rate.",
            ));
        }

        Ok((train_edges_number, valid_edges_number))
    }

    fn holdout(
        &self,
        random_state: EdgeT,
        train_size: f64,
        include_all_edge_types: bool,
        user_condition: impl Fn(EdgeT, NodeT, NodeT, Option<EdgeTypeT>) -> bool,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        let (_, valid_edges_number) =
            self.get_holdouts_edges_number(train_size, include_all_edge_types)?;

        let pb1 = get_loading_bar(
            verbose,
            "Picking validation edges",
            valid_edges_number as usize,
        );

        // generate and shuffle the indices of the edges
        let mut rng = SmallRng::seed_from_u64(random_state ^ SEED_XOR as EdgeT);
        let mut edge_indices: Vec<EdgeT> = (0..self.get_edges_number()).collect();
        edge_indices.shuffle(&mut rng);

        let mut valid_edges_bitmap = RoaringTreemap::new();
        let mut last_length = 0;

        for (edge_id, (src, dst, edge_type)) in edge_indices
            .iter()
            .cloned()
            .map(|edge_id| (edge_id, self.get_edge_triple(edge_id)))
        {
            // If the graph is undirected and we have extracted an edge that is a
            // simmetric one, we can skip this iteration.
            if !self.directed && src > dst {
                continue;
            }

            // We stop adding edges when we have reached the minimum amount.
            if user_condition(edge_id, src, dst, edge_type) {
                // Compute the forward edge ids that are required.
                valid_edges_bitmap.extend(self.compute_edge_ids_vector(
                    edge_id,
                    src,
                    dst,
                    include_all_edge_types,
                ));

                // If the graph is undirected
                if !self.directed {
                    // we compute also the backward edge ids that are required.
                    valid_edges_bitmap.extend(self.compute_edge_ids_vector(
                        self.get_unchecked_edge_id(dst, src, edge_type),
                        dst,
                        src,
                        include_all_edge_types,
                    ));
                }
                pb1.inc(valid_edges_bitmap.len() - last_length);
                last_length = valid_edges_bitmap.len();
            }

            // We stop the iteration when we found all the edges.
            if valid_edges_bitmap.len() >= valid_edges_number {
                break;
            }
        }

        if valid_edges_bitmap.len() < valid_edges_number {
            let actual_valid_edges_number = valid_edges_bitmap.len();
            let valid_rate = 1.0 - train_size;
            let actual_valid_rate =
                actual_valid_edges_number as f64 / self.get_edges_number() as f64;
            let actual_train_size = 1.0 - actual_valid_rate;
            return Err(format!(
                concat!(
                    "With the given configuration for the holdout, it is not possible to ",
                    "generate a validation set composed of {valid_edges_number} edges from the current graph.\n",
                    "The validation set can be composed of at most {actual_valid_edges_number} edges.\n",
                    "The actual train/valid split rates, with the current configuration,",
                    "would not be {train_size}/{valid_rate} but {actual_train_size}/{actual_valid_rate}.\n",
                    "If you really want to do this, you can pass the argument:\n",
                    "train_size: {actual_train_size}\n",
                    "Before proceeding, consider what is your experimental setup goal and ",
                    "the possible bias and validation problems that this choice might cause."
                ),
                valid_edges_number=valid_edges_number,
                actual_valid_edges_number=actual_valid_edges_number,
                train_size=train_size,
                valid_rate=valid_rate,
                actual_train_size=actual_train_size,
                actual_valid_rate=actual_valid_rate
            ));
        }

        // Creating the loading bar for the building of both the training and validation.
        let pb_valid = get_loading_bar(
            verbose,
            "Building the valid partition",
            valid_edges_bitmap.len() as usize,
        );
        let pb_train = get_loading_bar(
            verbose,
            "Building the train partition",
            (self.get_edges_number() - valid_edges_bitmap.len()) as usize,
        );

        Ok((
            Graph::build_graph(
                (0..self.get_edges_number())
                    .filter(|edge_id| !valid_edges_bitmap.contains(*edge_id))
                    .progress_with(pb_train)
                    .map(|edge_id| Ok(self.get_edge_quadruple(edge_id))),
                self.get_edges_number() - valid_edges_bitmap.len() as EdgeT,
                self.nodes.clone(),
                self.node_types.clone(),
                match &self.edge_types {
                    Some(ets) => Some(ets.vocabulary.clone()),
                    None => None,
                },
                self.directed,
                format!("{} training", self.name.clone()),
                false,
            )?,
            Graph::build_graph(
                valid_edges_bitmap
                    .iter()
                    .progress_with(pb_valid)
                    .map(|edge_id| Ok(self.get_edge_quadruple(edge_id))),
                valid_edges_bitmap.len() as EdgeT,
                self.nodes.clone(),
                self.node_types.clone(),
                match &self.edge_types {
                    Some(ets) => Some(ets.vocabulary.clone()),
                    None => None,
                },
                self.directed,
                format!("{} testing", self.name.clone()),
                false,
            )?,
        ))
    }

    /// Returns holdout for training ML algorithms on the graph structure.
    ///
    /// The holdouts returned are a tuple of graphs. The first one, which
    /// is the training graph, is garanteed to have the same number of
    /// graph components as the initial graph. The second graph is the graph
    /// meant for testing or validation of the algorithm, and has no garantee
    /// to be connected. It will have at most (1-train_size) edges,
    /// as the bound of connectivity which is required for the training graph
    /// may lead to more edges being left into the training partition.
    ///
    /// In the option where a list of edge types has been provided, these
    /// edge types will be those put into the validation set.
    ///
    /// # Arguments
    ///
    /// * `random_state`: NodeT - The random_state to use for the holdout,
    /// * `train_size`: f64 - Rate target to reserve for training.
    /// * `edge_types`: Option<Vec<String>> - Edge types to be selected for in the validation set.
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    ///
    pub fn connected_holdout(
        &self,
        random_state: EdgeT,
        train_size: f64,
        edge_types: Option<Vec<String>>,
        include_all_edge_types: bool,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        if train_size <= 0.0 || train_size >= 1.0 {
            return Err(String::from("Train rate must be strictly between 0 and 1."));
        }

        let edge_type_ids = if let Some(ets) = edge_types {
            Some(
                self.translate_edge_types(ets)?
                    .into_iter()
                    .collect::<HashSet<EdgeTypeT>>(),
            )
        } else {
            None
        };

        let tree = self.spanning_tree(
            random_state,
            include_all_edge_types,
            &edge_type_ids,
            verbose,
        );

        let edge_factor = if self.is_directed() { 1 } else { 2 };
        let train_edges_number = (self.get_edges_number() as f64 * train_size) as EdgeT;
        let mut valid_edges_number = (self.get_edges_number() as f64 * (1.0 - train_size)) as EdgeT;

        if let Some(etis) = &edge_type_ids {
            let selected_edges_number: EdgeT = etis
                .iter()
                .map(|et| self.get_edge_type_number(*et) as EdgeT)
                .sum();
            valid_edges_number = (selected_edges_number as f64 * (1.0 - train_size)) as EdgeT;
        }

        if tree.len() * edge_factor > train_edges_number {
            return Err(format!(
                concat!(
                    "The given spanning tree of the graph contains {} edges ",
                    "that is more than the required training edges number {}.\n",
                    "This makes impossible to create a validation set using ",
                    "{} edges.\nIf possible, you should increase the ",
                    "train_size parameter which is currently equal to ",
                    "{}.\nThe deny map, by itself, is requiring at least ",
                    "a train rate of {}."
                ),
                tree.len() * edge_factor,
                train_edges_number,
                valid_edges_number,
                train_size,
                (tree.len() * edge_factor) as f64 / train_edges_number as f64
            ));
        }

        self.holdout(
            random_state,
            train_size,
            include_all_edge_types,
            |edge_id, _, _, edge_type| {
                // The tree must not contain the provided edge ID
                !tree.contains(edge_id)
                // And the edge type of the edge ID is within the provided edge type
                    && match &edge_type_ids {
                        Some(etis) => {
                            etis.contains(&edge_type.unwrap())
                        },
                        None => true
                    }
            },
            verbose,
        )
    }

    /// Returns random holdout for training ML algorithms on the graph edges.
    ///
    /// The holdouts returned are a tuple of graphs. In neither holdouts the
    /// graph connectivity is necessarily preserved. To maintain that, use
    /// the method `connected_holdout`.
    ///
    /// # Arguments
    ///
    /// * `random_state`: NodeT - The random_state to use for the holdout,
    /// * `train_size`: f64 - rate target to reserve for training
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `edge_types`: Option<Vec<String>> - The edges to include in validation set.
    /// * `min_number_overlaps`: Option<usize> - The minimum number of overlaps to include the edge into the validation set.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn random_holdout(
        &self,
        random_state: EdgeT,
        train_size: f64,
        include_all_edge_types: bool,
        edge_types: Option<Vec<String>>,
        min_number_overlaps: Option<EdgeT>,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        let edge_type_ids = if let Some(ets) = edge_types {
            Some(
                self.translate_edge_types(ets)?
                    .into_iter()
                    .collect::<HashSet<EdgeTypeT>>(),
            )
        } else {
            None
        };
        if min_number_overlaps.is_some() && !self.is_multigraph() {
            return Err("Current graph is not a multigraph!".to_string());
        }
        self.holdout(
            random_state,
            train_size,
            include_all_edge_types,
            |_, src, dst, edge_type| {
                // If a list of edge types was provided and the edge type
                // of the current edge is not within the provided list,
                // we skip the current edge.
                if let Some(etis) = &edge_type_ids {
                    if !etis.contains(&edge_type.unwrap()) {
                        return false;
                    }
                }
                // If a minimum number of overlaps was provided and the current
                // edge has not the required minimum amount of overlaps.
                if let Some(mno) = min_number_overlaps {
                    if self.get_unchecked_edge_types_number_from_tuple(src, dst) < mno {
                        return false;
                    }
                }
                // Otherwise we accept the provided edge for the validation set
                true
            },
            verbose,
        )
    }

    /// Returns subgraph with given number of nodes.
    ///
    /// This method creates a subset of the graph starting from a random node
    /// sampled using given random_state and includes all neighbouring nodes until
    /// the required number of nodes is reached. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    ///
    ///
    /// # Arguments
    ///
    /// * `random_state`: usize - Random random_state to use.
    /// * `nodes_number`: usize - Number of nodes to extract.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn random_subgraph(
        &self,
        random_state: usize,
        nodes_number: NodeT,
        verbose: bool,
    ) -> Result<Graph, String> {
        if nodes_number <= 1 {
            return Err(String::from("Required nodes number must be more than 1."));
        }
        let not_singleton_nodes_number = self.get_not_singleton_nodes_number();
        if nodes_number > not_singleton_nodes_number {
            return Err(format!(
                concat!(
                    "Required number of nodes ({}) is more than available ",
                    "number of nodes ({}) that have edges in current graph."
                ),
                nodes_number, not_singleton_nodes_number
            ));
        }

        // Creating the loading bars
        let pb1 = get_loading_bar(verbose, "Sampling nodes subset", nodes_number as usize);
        let pb2 = get_loading_bar(verbose, "Computing subgraph edges", nodes_number as usize);
        let pb3 = get_loading_bar(
            verbose,
            "Building subgraph",
            self.get_edges_number() as usize,
        );

        // Creating the random number generator
        let mut rnd = SmallRng::seed_from_u64((random_state ^ SEED_XOR) as u64);

        // Nodes indices
        let mut nodes: Vec<NodeT> = (0..self.get_nodes_number()).collect();

        // Shuffling the components using the given random_state.
        nodes.shuffle(&mut rnd);

        // Initializing stack and set of nodes
        let mut unique_nodes = RoaringBitmap::new();
        let mut stack: Vec<NodeT> = Vec::new();

        // We iterate on the components
        'outer: for node in nodes.iter() {
            // If the current node is a trap there is no need to continue with the current loop.
            if self.is_node_trap(*node) {
                continue;
            }
            stack.push(*node);
            while !stack.is_empty() {
                let src = stack.pop().unwrap();
                for dst in self.get_source_destinations_range(src) {
                    if !unique_nodes.contains(dst) && src != dst {
                        stack.push(dst);
                    }

                    unique_nodes.insert(*node);
                    unique_nodes.insert(dst);
                    pb1.inc(2);

                    // If we reach the desired number of unique nodes we can stop the iteration.
                    if unique_nodes.len() as NodeT >= nodes_number {
                        break 'outer;
                    }
                }
            }
        }

        pb1.finish();

        let edges_bitmap =
            RoaringTreemap::from_iter(unique_nodes.iter().progress_with(pb2).flat_map(|src| {
                let (min_edge_id, max_edge_id) = self.get_destinations_min_max_edge_ids(src);
                (min_edge_id..max_edge_id)
                    .filter(|edge_id| unique_nodes.contains(self.get_destination(*edge_id)))
                    .collect::<Vec<EdgeT>>()
            }));

        Graph::build_graph(
            edges_bitmap
                .iter()
                .progress_with(pb3)
                .map(|edge_id| Ok(self.get_edge_quadruple(edge_id))),
            edges_bitmap.len() as EdgeT,
            self.nodes.clone(),
            self.node_types.clone(),
            match &self.edge_types {
                Some(ets) => Some(ets.vocabulary.clone()),
                None => None,
            },
            self.directed,
            format!("{} subgraph", self.name.clone()),
            false,
        )
    }

    /// Returns subgraph with given set of edge types.
    ///
    /// This method creates a subset of the graph by keeping also the edges
    /// of the given edge types.
    ///
    /// # Arguments
    ///
    /// * edge_types: Vec<String> - Vector of edge types to keep in the graph.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn edge_types_subgraph(
        &self,
        edge_types: Vec<String>,
        verbose: bool,
    ) -> Result<Graph, String> {
        if edge_types.is_empty() {
            return Err(String::from(
                "Required edge types must be a non-empty list.",
            ));
        }

        let edge_type_ids: HashSet<EdgeTypeT> = self
            .translate_edge_types(edge_types)?
            .iter()
            .cloned()
            .collect::<HashSet<EdgeTypeT>>();

        let pb = get_loading_bar(
            verbose,
            "Creating subgraph with given edge types",
            self.get_edges_number() as usize,
        );

        let edges_number = edge_type_ids
            .iter()
            .map(|et| self.get_edge_type_number(*et) as EdgeT)
            .sum();

        Graph::build_graph(
            (0..self.get_edges_number())
                .progress_with(pb)
                .filter_map(|edge_id| match &self.edge_types {
                    Some(ets) => match edge_type_ids.contains(&ets.ids[edge_id as usize]) {
                        true => Some(self.get_edge_quadruple(edge_id)),
                        false => None,
                    },
                    None => None,
                })
                .map(Ok),
            edges_number,
            self.nodes.clone(),
            self.node_types.clone(),
            match &self.edge_types {
                Some(ets) => Some(ets.vocabulary.clone()),
                None => None,
            },
            self.directed,
            format!("{} edge type subgraph", self.name.clone()),
            false,
        )
    }
}
