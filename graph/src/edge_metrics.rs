use super::types::*;
use super::*;
use num_traits::Pow;

/// # Properties and measurements of the graph
impl Graph {
    /// Returns the minumum preferential attachment score.
    ///
    /// # Safety
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_min_preferential_attachment(&self) -> f64 {
        (self.get_unchecked_min_node_degree() as f64).pow(2)
    }

    /// Returns the maximum preferential attachment score.
    ///
    /// # Safety
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_max_preferential_attachment(&self) -> f64 {
        (self.get_unchecked_max_node_degree() as f64).pow(2)
    }

    /// Returns the preferential attachment.
    ///
    /// # Arguments
    ///
    /// * `one`: NodeT - Integer ID of the first node.
    /// * `two`: NodeT - Integer ID of the second node.
    /// * `normalize`: bool - Whether to normalize within 0 to 1.
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_preferential_attachment(
        &self,
        one: NodeT,
        two: NodeT,
        normalize: bool,
    ) -> f64 {
        let mut preferential_attachment_score = self.get_unchecked_node_degree_from_node_id(one)
            as f64
            * self.get_unchecked_node_degree_from_node_id(two) as f64;
        if normalize {
            let min_preferential_attachment_score =
                self.get_unchecked_min_preferential_attachment();
            let max_preferential_attachment_score =
                self.get_unchecked_max_preferential_attachment();
            preferential_attachment_score = (preferential_attachment_score
                - min_preferential_attachment_score)
                / (max_preferential_attachment_score - min_preferential_attachment_score);
        }
        preferential_attachment_score
    }

    /// Returns the preferential attachment.
    ///
    /// # Arguments
    ///
    /// * `one`: NodeT - Integer ID of the first node.
    /// * `two`: NodeT - Integer ID of the second node.
    /// * `normalize`: bool - Whether to normalize by the square of maximum degree.
    ///
    /// # Raises
    /// * If eithert of the node IDs are higher than the number of nodes in the graph.
    pub fn get_preferential_attachment(
        &self,
        one: NodeT,
        two: NodeT,
        normalize: bool,
    ) -> Result<f64, String> {
        Ok(unsafe {
            self.get_unchecked_preferential_attachment(
                self.validate_node_id(one)?,
                self.validate_node_id(two)?,
                normalize,
            )
        })
    }

    /// Returns the Jaccard index for the two given nodes.
    ///
    /// # Arguments
    ///
    /// * `one`: NodeT - Integer ID of the first node.
    /// * `two`: NodeT - Integer ID of the second node.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The Jaccard Index between node 1 and node 2 is {}", unsafe{ graph.get_unchecked_jaccard_coefficient(1, 2) });
    /// ```
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_jaccard_coefficient(&self, one: NodeT, two: NodeT) -> f64 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(one, two)
            .count() as f64
            / self
                .iter_unchecked_neighbour_node_ids_union_from_source_node_ids(one, two)
                .count() as f64
    }

    /// Returns the Jaccard index for the two given nodes.
    ///
    /// # Arguments
    ///
    /// * `one`: NodeT - Integer ID of the first node.
    /// * `two`: NodeT - Integer ID of the second node.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The Jaccard Index between node 1 and node 2 is {}", graph.get_jaccard_coefficient(1, 2).unwrap());
    /// ```
    ///
    /// # Raises
    /// * If eithert of the node IDs are higher than the number of nodes in the graph.
    pub fn get_jaccard_coefficient(&self, one: NodeT, two: NodeT) -> Result<f64, String> {
        Ok(unsafe {
            self.get_unchecked_jaccard_coefficient(
                self.validate_node_id(one)?,
                self.validate_node_id(two)?,
            )
        })
    }

    /// Returns the Adamic/Adar Index for the given pair of nodes.
    ///
    /// # Arguments
    ///
    /// * `one`: NodeT - Integer ID of the first node.
    /// * `two`: NodeT - Integer ID of the second node.
    ///
    /// # Implementation details
    /// Since the Adamic/Adar Index is only defined for graph not containing
    /// node traps (nodes without any outbound edge) and must support all kind
    /// of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_adamic_adar_index(&self, one: NodeT, two: NodeT) -> f64 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(one, two)
            .map(|node_id| self.get_unchecked_node_degree_from_node_id(node_id))
            .filter(|&node_degree| node_degree > 1)
            .map(|node_degree| 1.0 / (node_degree as f64).ln())
            .sum()
    }

    /// Returns the Adamic/Adar Index for the given pair of nodes.
    ///
    /// # Arguments
    ///
    /// * `one`: NodeT - Integer ID of the first node.
    /// * `two`: NodeT - Integer ID of the second node.
    ///
    /// # Implementation details
    /// Since the Adamic/Adar Index is only defined for graph not containing
    /// node traps (nodes without any outbound edge) and must support all kind
    /// of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    /// # Raises
    /// * If eithert of the node IDs are higher than the number of nodes in the graph.
    pub fn get_adamic_adar_index(&self, one: NodeT, two: NodeT) -> Result<f64, String> {
        Ok(unsafe {
            self.get_unchecked_adamic_adar_index(
                self.validate_node_id(one)?,
                self.validate_node_id(two)?,
            )
        })
    }

    /// Returns the Resource Allocation Index for the given pair of nodes.
    ///
    /// # Arguments
    ///
    /// * `one`: NodeT - Integer ID of the first node.
    /// * `two`: NodeT - Integer ID of the second node.
    ///
    /// # References
    /// [T. Zhou, L. Lu, Y.-C. Zhang.
    /// Predicting missing links via local information.
    /// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
    ///
    /// # Implementation details
    /// Since the Resource Allocation Index is only defined for graph not
    /// containing node traps (nodes without any outbound edge) and
    /// must support all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_resource_allocation_index(&self, one: NodeT, two: NodeT) -> f64 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(one, two)
            .map(|node_id| self.get_unchecked_node_degree_from_node_id(node_id))
            .filter(|&node_degree| node_degree > 0)
            .map(|node_degree| 1.0 / node_degree as f64)
            .sum()
    }

    /// Returns the Resource Allocation Index for the given pair of nodes.
    ///
    /// # Arguments
    ///
    /// * `one`: NodeT - Integer ID of the first node.
    /// * `two`: NodeT - Integer ID of the second node.
    ///
    /// # References
    /// [T. Zhou, L. Lu, Y.-C. Zhang.
    /// Predicting missing links via local information.
    /// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
    ///
    /// # Implementation details
    /// Since the Resource Allocation Index is only defined for graph not
    /// containing node traps (nodes without any outbound edge) and
    /// must support all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Raises
    /// * If eithert of the node IDs are higher than the number of nodes in the graph.
    pub fn get_resource_allocation_index(&self, one: NodeT, two: NodeT) -> Result<f64, String> {
        Ok(unsafe {
            self.get_unchecked_resource_allocation_index(
                self.validate_node_id(one)?,
                self.validate_node_id(two)?,
            )
        })
    }
}