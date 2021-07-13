use elias_fano_rust::EliasFano;

use super::*;

#[derive(Debug, Clone)]
pub(crate) struct PropertyCache {
    pub(crate) min_edge_weight: Option<Result<WeightT>>,
    pub(crate) max_edge_weight: Option<Result<WeightT>>,
    pub(crate) total_edge_weight: Option<Result<f64>>,
    pub(crate) min_node_degree: Option<NodeT>,
    pub(crate) max_node_degree: Option<NodeT>,
    pub(crate) nodes_sorted_by_increasing_outbound_node_degree: Option<bool>,
    pub(crate) nodes_sorted_by_decreasing_outbound_node_degree: Option<bool>,
    pub(crate) most_central_node_id: Option<NodeT>,
    pub(crate) max_weighted_node_degree: Option<Result<f64>>,
    pub(crate) min_weighted_node_degree: Option<Result<f64>>,
    pub(crate) weighted_singleton_nodes_number: Option<Result<NodeT>>,
    pub(crate) trap_nodes_number: Option<NodeT>,
    pub(crate) selfloops_number: Option<EdgeT>,
    pub(crate) selfloops_number_unique: Option<NodeT>,
    pub(crate) singleton_nodes_with_selfloops_number: Option<NodeT>,
    pub(crate) connected_nodes_number: Option<NodeT>,
    pub(crate) unique_directed_edges_number: Option<EdgeT>,
    pub(crate) connected_nodes: Option<ConcurrentBitVec>,
    pub(crate) unique_sources: Option<EliasFano>
}

impl Default for PropertyCache {
    fn default() -> Self {
        PropertyCache {
            min_edge_weight: None,
            max_edge_weight: None,
            total_edge_weight: None,
            min_node_degree: None,
            max_node_degree: None,
            nodes_sorted_by_increasing_outbound_node_degree: None,
            nodes_sorted_by_decreasing_outbound_node_degree: None,
            most_central_node_id: None,
            max_weighted_node_degree: None,
            min_weighted_node_degree: None,
            weighted_singleton_nodes_number: None,
            trap_nodes_number: None,
            selfloops_number: None,
            selfloops_number_unique: None,
            singleton_nodes_with_selfloops_number: None,
            connected_nodes_number: None,
            connected_nodes: None,
            unique_directed_edges_number: None,
            unique_sources: None
        }
    }
}