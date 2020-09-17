/// Build WalkWeights object from provided kwargs
///
/// # Arguments
///
/// * py_kwargs: Option<&PyDict> - The kwargs provided by the user.
fn build_walk_weights(py_kwargs: Option<&PyDict>) -> PyResult<WalkWeights> {
    let mut weights = WalkWeights::default();
    if let Some(kwargs) = &py_kwargs {
        weights = to_python_exception!(weights.set_return_weight(extract_value!(
            kwargs,
            "return_weight",
            ParamsT
        )))?;
        weights = to_python_exception!(weights.set_explore_weight(extract_value!(
            kwargs,
            "explore_weight",
            ParamsT
        )))?;
        weights = to_python_exception!(weights.set_change_edge_type_weight(extract_value!(
            kwargs,
            "change_edge_type_weight",
            ParamsT
        )))?;
        weights = to_python_exception!(weights.set_change_node_type_weight(extract_value!(
            kwargs,
            "change_node_type_weight",
            ParamsT
        )))?;
    }
    Ok(weights)
}

/// Build SingleWalkParameters object from provided kwargs
///
/// # Arguments
///
/// * length: usize - the length of the walks.
/// * py_kwargs: Option<&PyDict> - The kwargs provided by the user.
fn build_single_walk_parameters(
    length: usize,
    py_kwargs: Option<&PyDict>,
) -> PyResult<SingleWalkParameters> {
    to_python_exception!(SingleWalkParameters::new(
        length,
        build_walk_weights(py_kwargs)?,
    ))
}

/// Build WalksParameters object from provided kwargs
///
/// # Arguments
///
/// * length: usize - the length of the walks.
/// * py_kwargs: Option<&PyDict> - The kwargs provided by the user.
fn build_walk_parameters(
    length: usize,
    start_node: NodeT,
    end_node: NodeT,
    py_kwargs: Option<&PyDict>,
    validate: bool,
) -> PyResult<WalksParameters> {
    let mut walks_parameters = to_python_exception!(WalksParameters::new(
        build_single_walk_parameters(length, py_kwargs)?,
        start_node,
        end_node,
    ))?;
    if let Some(kwargs) = &py_kwargs {
        if validate {
            validate_kwargs(
                kwargs,
                &[
                    "iterations",
                    "min_length",
                    "dense_nodes_mapping",
                    "return_weight",
                    "explore_weight",
                    "change_edge_type_weight",
                    "change_node_type_weight",
                    "verbose",
                    "seed",
                ],
            )?;
        }
        walks_parameters = to_python_exception!(walks_parameters.set_iterations(extract_value!(
            kwargs,
            "iterations",
            usize
        )))?;
        walks_parameters = walks_parameters.set_verbose(extract_value!(kwargs, "verbose", bool));
        walks_parameters = walks_parameters.set_seed(extract_value!(kwargs, "seed", usize));
        walks_parameters = to_python_exception!(walks_parameters.set_min_length(extract_value!(
            kwargs,
            "min_length",
            usize
        )))?;
        walks_parameters = walks_parameters.set_dense_nodes_mapping(
            extract_value!(kwargs, "dense_nodes_mapping", HashMap<NodeT, NodeT>),
        );
    }
    Ok(walks_parameters)
}

impl EnsmallenGraph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, length, *, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_nodes_mapping, seed, verbose)"]
    /// Return random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ---------------------
    /// length: int,
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// iterations: int = 1,
    ///     Number of cycles on the graphs to execute.
    /// min_length: int = 0,
    ///     Minimal length of the random walk. Will filter out smaller
    ///     random walks.
    /// return_weight: float = 1.0,
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_node_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_edge_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// dense_nodes_mapping: Dict[int, int],
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_nodes_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// seed: int,
    ///     Seed to use to reproduce the walks.
    /// verbose: int = True,
    ///     Wethever to show or not the loading bar of the walks.
    ///
    /// Returns
    /// ----------------------------
    /// List of list of walks containing the numeric IDs of nodes.
    ///
    fn walk(&self, length: usize, py_kwargs: Option<&PyDict>) -> PyResult<Vec<Vec<NodeT>>> {
        match build_walk_parameters(
            length,
            0,
            self.graph.get_not_trap_nodes_number(),
            py_kwargs,
            true,
        ) {
            Ok(walk_parameters) => match self.graph.walk(&walk_parameters) {
                Ok(w) => Ok(w),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            },
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }
}