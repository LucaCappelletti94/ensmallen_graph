use super::*;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, Clone)]
pub struct FuzzFeaturesHarnessParams {
    from_vec: FromVecHarnessParams,

}

pub fn features_harness(data: FromVecHarnessParams) -> Result<(), String> {
    let data_for_panic_handling1 = data.clone();
    let data_for_panic_handling2 = data.clone();
    let data_for_signal_handling = data.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_from_vec(Some(info), data_for_panic_handling1.clone(), None);
    }));


    register_handler(libc::SIGABRT, abrt_handler, data_for_signal_handling);

    let mut graph = graph::Graph::from_string_unsorted(
        data.edges.into_iter(),
        data.nodes.map(|ns| ns.into_iter()),
        data.directed,
        data.directed_edge_list,
        "Fuzz Graph",
        data.ignore_duplicated_nodes,
        false,
        data.ignore_duplicated_edges,
        false,
        data.numeric_edge_types_ids,
        data.numeric_node_ids,
        data.numeric_edge_node_ids,
        data.numeric_node_types_ids,
        data.has_node_types,
        data.has_edge_types,
        data.has_edge_weights,
        true,
        true,
        true,
        data.verbose,
    )?;

    let graph_copy_for_panic_handling = graph.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_from_vec_once_loaded(
            Some(info),
            data_for_panic_handling2.clone(),
            graph_copy_for_panic_handling.clone()
        );
    }));



    // We ignore this error because we execute only the fuzzing to find
    // the panic situations that are NOT just errors, but unhandled errors.
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    
    Ok(())
}