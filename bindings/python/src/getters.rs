impl EnsmallenGraph {
    #[text_signature = "(self)"]
    /// Return the number of nodes in the graph.
    fn get_nodes_number(&self) -> usize {
        self.graph.get_nodes_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of non trap nodes in the graph.
    fn get_not_trap_nodes_number(&self) -> usize {
        self.graph.get_not_trap_nodes_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of edges in the graph.
    fn get_edges_number(&self) -> usize {
        self.graph.get_edges_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of edges types in the graph.
    ///
    /// This method will include, if found necessary by a missing value,
    /// also the default edge type in the count of total edge types.
    ///
    fn get_edge_types_number(&self) -> usize {
        self.graph.get_edge_types_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of edges in the graph.
    ///
    /// This method will include, if found necessary by a missing value,
    /// also the default node type in the count of total node types.
    ///
    fn get_node_types_number(&self) -> usize {
        self.graph.get_node_types_number()
    }

    #[text_signature = "($self, node)"]
    /// Return boolean representing if given node is a trap.
    ///
    /// A trap node is a node with no outbounds edges.
    ///
    /// Parameters
    /// ---------------------
    /// node: int,
    ///     Node ID to search if it's a trap.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given node is a trap.
    ///
    fn is_node_trap(&self, node: NodeT) -> bool {
        self.graph.is_node_trap(node)
    }

    #[text_signature = "($self, edge)"]
    /// Return boolean representing if given edge is a trap.
    ///
    /// A trap edge is a edge with a destination node that is a trap node.
    ///
    /// Parameters
    /// ---------------------
    /// node: int,
    ///     Node ID to search if it's a trap.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given edge is a trap.
    ///
    fn is_edge_trap(&self, edge: EdgeT) -> bool {
        self.graph.is_edge_trap(edge)
    }

    #[text_signature = "($self, node)"]
    /// Return list of Node IDs of the neighbours of given node.
    ///
    /// Parameters
    /// ---------------------
    /// node: int,
    ///     Node ID to
    ///
    /// Returns
    /// ----------------------------
    /// List of Node IDs of the neighbouring nodes.
    ///
    fn get_node_neighbours(&self, node: NodeT) -> Vec<NodeT> {
        self.graph.get_node_neighbours(node)
    }

    #[text_signature = "($self, src, dst)"]
    /// Return boolean representing if given edge exists in graph.
    ///
    /// Parameters
    /// ---------------------
    /// src: int,
    ///     Node ID to use as source of given edge.
    /// dst: int,
    ///     Node ID to use as destination of given edge.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given edge exists in graph.
    ///
    fn has_edge(&self, src: NodeT, dst: NodeT) -> bool {
        self.graph.has_edge(src, dst)
    }

    #[text_signature = "($self)"]
    /// Return mapping from instance not trap nodes to dense range of nodes.
    ///
    /// Returns
    /// ----------------------------
    /// Dict with mapping from not trap nodes to dense range of nodes.
    ///
    fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.graph.get_dense_nodes_mapping()
    }

    #[getter]
    fn sources(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(gil, self.graph.sources().clone(), NodeT))
    }

    #[getter]
    fn destinations(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(
            gil,
            self.graph.destinations().clone(),
            NodeT
        ))
    }

    #[getter]
    fn nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.graph
            .nodes_mapping()
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    #[getter]
    fn nodes_reverse_mapping(&self) -> Vec<String> {
        self.graph.nodes_reverse_mapping().clone()
    }

    #[getter]
    fn unique_edges(&self) -> HashMap<(NodeT, NodeT), EdgeT> {
        self.graph
            .unique_edges()
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect()
    }

    #[getter]
    fn outbounds(&self) -> PyResult<Py<PyArray1<EdgeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(gil, self.graph.outbounds().clone(), EdgeT))
    }

    #[getter]
    fn weights(&self) -> PyResult<Option<Py<PyArray1<WeightT>>>> {
        Ok(match self.graph.weights().clone() {
            Some(w) => {
                let gil = pyo3::Python::acquire_gil();
                Some(to_nparray_1d!(gil, w, WeightT))
            }
            None => None,
        })
    }

    #[getter]
    fn node_types(&self) -> PyResult<Option<Py<PyArray1<NodeTypeT>>>> {
        Ok(match self.graph.node_types().clone() {
            Some(w) => {
                let gil = pyo3::Python::acquire_gil();
                Some(to_nparray_1d!(gil, w, NodeTypeT))
            }
            None => None,
        })
    }

    #[getter]
    fn node_types_mapping(&self) -> Option<HashMap<String, NodeTypeT>> {
        match self.graph.node_types_mapping() {
            None => None,
            Some(g) => Some(g.iter().map(|(k, v)| (k.clone(), *v)).collect()),
        }
    }

    #[getter]
    fn node_types_reverse_mapping(&self) -> Option<Vec<String>> {
        self.graph.node_types_reverse_mapping().clone()
    }

    #[getter]
    fn edge_types(&self) -> PyResult<Option<Py<PyArray1<EdgeTypeT>>>> {
        Ok(match self.graph.edge_types().clone() {
            Some(w) => {
                let gil = pyo3::Python::acquire_gil();
                Some(to_nparray_1d!(gil, w, EdgeTypeT))
            }
            None => None,
        })
    }

    #[getter]
    fn edge_types_mapping(&self) -> Option<HashMap<String, EdgeTypeT>> {
        match self.graph.edge_types_mapping() {
            None => None,
            Some(g) => Some(g.iter().map(|(k, v)| (k.clone(), *v)).collect()),
        }
    }

    #[getter]
    fn edge_types_reverse_mapping(&self) -> Option<Vec<String>> {
        self.graph.edge_types_reverse_mapping().clone()
    }

    #[text_signature = "($self, node_id)"]
    /// Return the id of the node type of the node.
    ///
    /// Parameters
    /// ---------------------
    /// node_id: int,
    ///     Numeric ID of the node.
    ///
    /// Returns
    /// ---------------------
    /// Id of the node type of the node.
    fn get_node_type_id(&self, node_id: NodeT) -> PyResult<NodeTypeT> {
        to_python_exception!(self.graph.get_node_type_id(node_id))
    }

    #[text_signature = "($self, edge_id)"]
    /// Return the id of the edge type of the edge.
    ///
    /// Parameters
    /// ---------------------
    /// edge_id: int,
    ///     Numeric ID of the edge.
    ///
    /// Returns
    /// ---------------------
    /// Id of the edge type of the edge.
    fn get_edge_type_id(&self, edge_id: EdgeT) -> PyResult<EdgeTypeT> {
        to_python_exception!(self.graph.get_edge_type_id(edge_id))
    }

    #[text_signature = "($self, src, dst)"]
    /// Return random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ---------------------
    /// edge_id: int,
    ///     Numeric ID of the edge.
    ///
    /// Returns
    /// ---------------------
    /// Return the id of the edge type of the edge.
    fn get_edge_id(&self, src: NodeT, dst: NodeT) -> PyResult<EdgeT> {
        to_python_exception!(self.graph.get_edge_id(src, dst))
    }

    #[text_signature = "($self)"]
    /// Return the count of how many time an edge type appears.
    fn get_edge_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
        to_python_exception!(self.graph.get_edge_type_counts())
    }

    #[text_signature = "($self)"]
    /// Return the count of how many time an node type appears.
    fn get_node_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
        to_python_exception!(self.graph.get_node_type_counts())
    }

    #[text_signature = "($self, k)"]
    /// Return vectors with the nodes and node types within the top k most common.
    ///
    /// Parameters
    /// --------------------------
    /// k: int,
    ///     Number of common node types to return.
    ///
    /// Returns
    /// --------------------------
    /// Tuple with node IDs and node types within k most common node types.
    fn get_top_k_nodes_by_node_type(
        &self,
        k: usize,
    ) -> PyResult<(Py<PyArray1<NodeT>>, Py<PyArray1<NodeTypeT>>)> {
        match self.graph.get_top_k_nodes_by_node_type(k) {
            Ok((nodes, node_types)) => {
                let gil = pyo3::Python::acquire_gil();
                Ok((
                    to_nparray_1d!(gil, nodes, NodeT),
                    to_nparray_1d!(gil, node_types, NodeTypeT),
                ))
            }
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "($self)"]
    /// Return vectors with the edges and ed ge types within the top k most common.
    ///
    /// Parameters
    /// --------------------------
    /// k: int,
    ///     Number of common edge types to return.
    ///
    /// Returns
    /// --------------------------
    /// Tuple with edge IDs and edge types within k most common edge types.
    fn get_top_k_edges_by_edge_type(
        &self,
        k: usize,
    ) -> PyResult<(Py<PyArray1<NodeT>>, Py<PyArray1<NodeTypeT>>)> {
        match self.graph.get_top_k_edges_by_edge_type(k) {
            Ok((edges, edge_types)) => {
                let gil = pyo3::Python::acquire_gil();
                Ok((
                    to_nparray_1d!(gil, edges, EdgeT),
                    to_nparray_1d!(gil, edge_types, EdgeTypeT),
                ))
            }
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }
}