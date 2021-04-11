use super::*;

/// # Unchecked Queries
/// The naming convection for unchecked methods follows `get_unchecked_X_by_Y`.
impl Graph {
    /// Returns the name of the node passed and HORRIBLY PANIC if the id is out
    /// of range.
    pub(crate) fn get_unchecked_node_name_by_node_id(&self, node_id: NodeT) -> String {
        self.nodes.unchecked_translate(node_id)
    }

    /// Returns option with the edge type of the given edge id.
    pub(crate) fn get_unchecked_edge_type_by_edge_id(&self, edge_id: EdgeT) -> Option<EdgeTypeT> {
        self.edge_types
            .as_ref()
            .and_then(|ets| ets.ids[edge_id as usize])
    }

    /// Returns option with the weight of the given edge id.
    pub(crate) fn get_unchecked_weight_by_edge_id(&self, edge_id: EdgeT) -> Option<WeightT> {
        self.weights.as_ref().map(|ws| ws[edge_id as usize])
    }

    /// Returns option with the node type of the given node id.
    pub(crate) fn get_unchecked_node_type_id_by_node_id(
        &self,
        node_id: NodeT,
    ) -> Option<Vec<NodeTypeT>> {
        self.node_types
            .as_ref()
            .and_then(|nts| nts.ids[node_id as usize].clone())
    }

    /// Returns node id raising a panic if used unproperly.
    pub(crate) fn get_unchecked_node_id_by_node_name(&self, node_name: &str) -> NodeT {
        *self.nodes.get(node_name).unwrap()
    }

    /// TODO: add doc
    pub(crate) fn get_unchecked_edge_type_id_by_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> Option<EdgeTypeT> {
        match (&self.edge_types, edge_type_name) {
            (Some(ets), Some(et)) => ets.get(et).copied(),
            _ => None,
        }
    }

    /// TODO: add doc
    pub(crate) fn get_unchecked_edge_type_name_by_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Option<String> {
        match (&self.edge_types, edge_type_id) {
            (Some(ets), Some(et)) => Some(ets.unchecked_translate(et)),
            _ => None,
        }
    }

    /// Return number of edges of the given edge type without checks.
    ///
    /// # Arguments
    ///
    /// * edge_type: Option<EdgeTypeT> - The edge type to retrieve count of.
    ///
    pub(crate) fn get_unchecked_edge_count_by_edge_type_id(
        &self,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        match (&self.edge_types, edge_type) {
            (Some(ets), None) => ets.get_unknown_count(),
            (Some(ets), Some(et)) => ets.counts[et as usize],
            _ => unreachable!("The current graph instance does not have edge types!"),
        }
    }

    /// Return number of nodes of the given node type without checks.
    ///
    /// # Arguments
    ///
    /// * node_type: Option<NodeTypeT> - The node type to retrieve count of.
    ///
    pub(crate) fn get_unchecked_node_count_by_node_type_id(
        &self,
        node_type: Option<NodeTypeT>,
    ) -> NodeT {
        match (&self.node_types, node_type) {
            (Some(nts), None) => nts.get_unknown_count(),
            (Some(nts), Some(nt)) => nts.counts[nt as usize],
            _ => unreachable!("The current graph instance does not have node types!"),
        }
    }

    /// Return (subsampled) vector of destinations of given node.
    ///
    /// If the max neighbours parameter is given, and is smaller than the
    /// number of the neighbours of the given node, the subsampling
    /// mechanism is given.
    ///
    /// # Arguments
    /// `node`: NodeT - Node whose neighbours are to return.
    /// `random_state`: u64 - Random state to subsample neighbours.
    /// `max_neighbours`: &Option<NodeT> - Optionally number of neighbours to consider.
    pub(crate) fn get_unchecked_node_destinations_by_node_id(
        &self,
        node: NodeT,
        random_state: u64,
        max_neighbours: Option<NodeT>,
    ) -> Vec<NodeT> {
        let (min_edge_id, max_edge_id, destinations, _) =
            self.get_node_edges_and_destinations(max_neighbours, random_state, node);
        self.get_destinations_slice(min_edge_id, max_edge_id, node, &destinations)
            .to_owned()
    }

    /// Return edge ID without any checks for given tuple of nodes and edge type.
    ///
    /// This method will cause a panic if used improperly when it is not certain
    /// that the edge exists.
    ///
    /// # Arguments
    /// `src`: NodeT - Source node of the edge.
    /// `dst`: NodeT - Destination node of the edge.
    /// `edge_type`: Option<EdgeTypeT> - Edge Type of the edge.
    pub(crate) fn get_unchecked_edge_id_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        self.edge_types.as_ref().map_or_else(
            || self.get_unchecked_edge_id_from_tuple(src, dst),
            |ets| {
                self.get_unchecked_edge_ids_range(src, dst)
                    // The vectors of the edge types can only have one element.
                    .find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
                    .unwrap()
            },
        )
    }

    /// Returns range of multigraph minimum and maximum edge ids with same source and destination nodes and different edge type.
    ///
    /// # Arguments
    ///
    /// * `src` - Source node of the edge.
    /// * `dst` - Destination node of the edge.
    ///
    pub(crate) fn get_unchecked_edge_ids_range(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> impl Iterator<Item = EdgeT> {
        let (min_edge_id, max_edge_id) = self.get_unchecked_minmax_edge_ids_by_node_ids(src, dst);
        min_edge_id..max_edge_id
    }

    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_unchecked_minmax_edge_ids_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> (EdgeT, EdgeT) {
        (
            self.get_unchecked_edge_id_from_tuple(src, dst),
            self.get_unchecked_edge_id_from_tuple(src, dst + 1),
        )
    }

    /// Return the number of edges between the given source and destination nodes.
    ///
    /// This might be thought as the degree of an edge in a multigraph.
    /// On non-multigraph this trivially return 1 on existing edges and 0 on
    /// the non-existing ones.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_unchecked_edge_degreee_by_node_ids(&self, src: NodeT, dst: NodeT) -> EdgeT {
        let (min_edge_id, max_edge_id) = self.get_unchecked_minmax_edge_ids_by_node_ids(src, dst);
        max_edge_id - min_edge_id
    }
}