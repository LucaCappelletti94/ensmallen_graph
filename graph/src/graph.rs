//! A graph representation optimized for executing random walks on huge graphs.
use super::*;
use elias_fano_rust::EliasFano;
use rayon::prelude::*;
use std::collections::HashMap;

/// A graph representation optimized for executing random walks on huge graphs.
///
/// This class should be initialized using the two constructors:
/// `graph::Graph::new_directed` or `graph::Graph::new_undirected`
///
/// # Examples
///
#[derive(Clone, Debug)]
pub struct Graph {
    // properties
    /// if the graph is directed or undirected
    pub(crate) directed: bool,
    /// Number of nodes that have at least a self-loop.
    /// This means that if a nodes has multiples self-loops they will be count as one.
    pub(crate) unique_self_loop_number: NodeT,
    /// Number of self-loop edges. This counts multiple times eventual multi-graph self-loops.
    pub(crate) self_loop_number: EdgeT,
    /// Number of nodes that have at least an edge inbound or outbound.
    pub(crate) not_singleton_nodes_number: NodeT,
    /// Number of singleton nodes that have a self-loop
    pub(crate) singleton_nodes_with_self_loops_number: NodeT,
    /// How many unique edges the graph has (excluding the multi-graph ones)
    pub(crate) unique_edges_number: EdgeT,
    /// Vector of destinations to execute fast walks if required.
    pub(crate) destinations: Option<Vec<NodeT>>,
    /// Vector of sources to execute fast link prediction sequences if required.
    pub(crate) sources: Option<Vec<NodeT>>,
    /// Vector of outbounds to execute fast walks if required.
    pub(crate) outbounds: Option<Vec<EdgeT>>,
    // Hashmap of cached destinations to execute faster walks if required.
    pub(crate) cached_destinations: Option<HashMap<NodeT, Vec<NodeT>>>,
    /// Graph name
    pub(crate) name: String,

    /// The main datastructure where all the edges are saved
    /// in the endoced form ((src << self.node_bits) | dst) this allows us to do almost every
    /// operation in O(1) without decompressing the data.
    pub(crate) edges: EliasFano,
    /// How many bits are needed to save a node.
    pub(crate) node_bits: u8,
    /// The mask used to extract the dst value form an encoded edge.
    /// This is saved for speed sake. It's equivalent to (1 << self.node_bits) - 1;
    pub(crate) node_bit_mask: u64,
    /// Vocabulary that save the mappings from string to index of every node
    pub(crate) nodes: Vocabulary<NodeT>,
    pub(crate) unique_sources: EliasFano,

    /// Optional vector of the weights of every edge.
    /// `weights[10]` return the weight of the edge with edge_id 10
    pub(crate) weights: Option<Vec<WeightT>>,
    /// Vocabulary that save the mappings from string to index of every node type
    pub(crate) node_types: Option<NodeTypeVocabulary>,
    // This is the next attribute that will be embedded inside of edges once
    // the first refactoring is done
    /// Vocabulary that save the mappings from string to index of every edge type
    pub(crate) edge_types: Option<EdgeTypeVocabulary>,
}

/// # Graph utility methods
impl Graph {
    pub fn new<S: Into<String>>(
        directed: bool,
        unique_self_loop_number: NodeT,
        self_loop_number: EdgeT,
        not_singleton_nodes_number: NodeT,
        singleton_nodes_with_self_loops_number: NodeT,
        unique_edges_number: EdgeT,
        edges: EliasFano,
        unique_sources: EliasFano,
        nodes: Vocabulary<NodeT>,
        node_bit_mask: EdgeT,
        node_bits: u8,
        edge_types: Option<EdgeTypeVocabulary>,
        name: S,
        weights: Option<Vec<WeightT>>,
        node_types: Option<NodeTypeVocabulary>,
    ) -> Graph {
        Graph {
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            unique_edges_number,
            edges,
            unique_sources,
            node_bit_mask,
            node_bits,
            weights,
            node_types: node_types.map(|nts| nts.set_numeric_ids(false)),
            edge_types: edge_types.map(|ets| ets.set_numeric_ids(false)),
            nodes: nodes.set_numeric_ids(false),
            sources: None,
            destinations: None,
            outbounds: None,
            cached_destinations: None,
            name: name.into(),
        }
    }

    /// Return true if given graph has any edge overlapping with current graph.
    ///
    /// # Arguments
    ///
    /// * other: Graph - The graph to check against.
    ///
    pub fn overlaps(&self, other: &Graph) -> Result<bool, String> {
        Ok(match self.is_compatible(other)? {
            true => other
                .get_edges_par_triples(other.directed)
                .any(|(_, src, dst, et)| self.has_edge_with_type(src, dst, et)),
            false => other
                .get_edges_par_string_triples(other.directed)
                .any(|(_, src, dst, et)| self.has_edge_with_type_by_node_names(&src, &dst, et.as_ref())),
        })
    }

    /// Return true if given graph edges are all contained within current graph.
    ///
    /// # Arguments
    ///
    /// * other: Graph - The graph to check against.
    ///
    pub fn contains(&self, other: &Graph) -> Result<bool, String> {
        Ok(match self.is_compatible(other)? {
            true => other
                .get_edges_par_triples(other.directed)
                .all(|(_, src, dst, et)| self.has_edge_with_type(src, dst, et)),
            false => other
                .get_edges_par_string_triples(other.directed)
                .all(|(_, src, dst, et)| self.has_edge_with_type_by_node_names(&src, &dst, et.as_ref())),
        })
    }
}
