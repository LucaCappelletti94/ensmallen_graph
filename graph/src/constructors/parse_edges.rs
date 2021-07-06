use super::*;
use elias_fano_rust::{ConcurrentEliasFanoBuilder, EliasFano};
use rayon::prelude::*;
use std::cmp::Ordering;

macro_rules! create_edge_list {
    (
        $eis:expr,
        $nodes:expr,
        $node_method:expr,
        $edge_types_vocabulary:expr,
        $edge_types_method:expr,
        ($($input_tuple:ident),*),
        ($($results:ident),*),
        ($($default:expr),*),
        $directed:expr,
        $complete:expr,
        $duplicates:expr
    ) => {{
        // Create the edge type parser
        let mut edge_type_parser = EdgeTypeParser::new($edge_types_vocabulary);
        // Create the node parser
        let mut node_parser = EdgeNodeNamesParser::new($nodes);
        // Collecting the edges into a mutable vector of tuples
        // collecting exclusively what needs to be collected.
        let mut unsorted_edge_list = $eis.into_iter().flat_map(|ei| {
            // If the provided edge list is either
            // of a directed graph, hence there is no need in the first place to
            // create the edges in the opposite direction, or alternatively
            // the user has specified that the edge list is already complete
            // hence there is no need to create the inverse edges.
            let ei = ei.method_caller($edge_types_method, &mut edge_type_parser);
            let ei = ei.method_caller($node_method, &mut node_parser);
            if $directed || $complete {
                ei.map(|line| match line {
                    Ok((src, dst, edge_type, weight)) => unsafe { Ok((src, dst, $($input_tuple,)*)) },
                    Err(e) => Err(e)
                }).collect::<Vec<Result<_>>>()
            } else {
                ei.flat_map(|line| match line {
                    Ok((src, dst, edge_type, weight)) => unsafe {
                        if src == dst {
                            vec![Ok((src, dst, $($input_tuple,)*))]
                        } else {
                            vec![
                                Ok((src, dst, $($input_tuple,)*)),
                                Ok((dst, src, $($input_tuple,)*)),
                            ]
                        }
                    },
                    Err(e) => vec![Err(e)]
                })
                .collect::<Vec<Result<_>>>()
            }
        }).collect::<Result<Vec<_>>>()?;
        // Sorting the vector using a par sort, which is:
        // - unstable because we do not care for changing order of equal values
        // - requires a by because we have weights in the mix.
        unsorted_edge_list.par_sort_unstable_by(
            |v1, v2| {
                v1.partial_cmp(&v2).unwrap_or(Ordering::Greater)
            },
        );
        // Removes duplicated edges.
        if $duplicates {
            unsorted_edge_list.dedup_by(|v1, v2| {
                v1.partial_cmp(&v2).unwrap_or(Ordering::Greater) == Ordering::Equal
            });
        }
        // Get the number of nodes and edges.
        let edges_number = unsorted_edge_list.len();
        let nodes_number = $nodes.len();
        // First we collect the weights and edge types
        $(
            let mut $results = vec![$default; edges_number];
        )*
        // We also create the builder for the elias fano
        let node_bits = get_node_bits(nodes_number as NodeT);
        let maximum_edges_number = encode_max_edge(nodes_number as NodeT, node_bits);
        let elias_fano_builder = ConcurrentEliasFanoBuilder::new(
            edges_number as u64,
            maximum_edges_number
        )?;
        // Parsing and building edge list objects
        unsorted_edge_list
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (src, dst, $($input_tuple),*))| {
                elias_fano_builder.set(i as u64, encode_edge(src, dst, node_bits));
                $(
                    $results[i as usize] = $input_tuple;
                )*
            });
        // Finalizing the edges structure constructor
        let edges = elias_fano_builder.build();
        // Return the computed values
        (edges, $($results),*)
    }}
}

fn parse_unsorted_edges(
    edges_iterators: Option<
        Vec<impl ParallelIterator<Item = Result<(String, String, Option<String>, WeightT)>>>,
    >,
    nodes: Vocabulary<NodeT>,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    has_edge_types: bool,
    has_edge_weights: bool,
    directed: bool,
    correct: Option<bool>,
    complete: Option<bool>,
    duplicates: Option<bool>,
    expected_edges_number: Option<usize>,
    numeric_edge_list_node_ids: Option<bool>,
    numeric_edge_list_edge_type_ids: Option<bool>,
) -> Result<(
    Vocabulary<NodeT>,
    EliasFano,
    Option<EdgeTypeVocabulary>,
    Option<Vec<WeightT>>,
)> {
    let correct = correct.unwrap_or(false);
    let complete = complete.unwrap_or(false);
    let duplicates = duplicates.unwrap_or(true);
    let numeric_edge_list_node_ids = numeric_edge_list_node_ids.unwrap_or(false);
    let numeric_edge_list_edge_type_ids = numeric_edge_list_edge_type_ids.unwrap_or(false);

    if edges_iterators.as_ref().map_or(true, |ei| ei.is_empty()) && edge_types_vocabulary.is_some()
    {
        return Err(concat!(
            "Edge types vocabulary was provided ",
            "but no edge list was given."
        )
        .to_string());
    }

    let has_edge_types = edge_types_vocabulary.is_some();

    if !has_edge_types && numeric_edge_list_edge_type_ids {
        return Err(concat!(
            "The numeric node list node type IDs parameter does not make sense ",
            "in the context where the node types have not been provided.\n",
            "If the node types within the nodes list are numeric, simply use ",
            "the numeric node types ids parameter."
        )
        .to_string());
    }

    let edge_types_method = match (
        has_edge_types,
        edge_types_vocabulary
            .as_ref()
            .map_or(true, |x| x.is_empty()),
        correct,
        numeric_edge_list_edge_type_ids,
    ) {
        (false, _, _, false) => EdgeTypeParser::ignore,
        (true, true, true, false) => EdgeTypeParser::parse_strings_unchecked,
        (true, true, false, false) => EdgeTypeParser::parse_strings,
        (true, false, true, false) => EdgeTypeParser::get_unchecked,
        (true, false, false, false) => EdgeTypeParser::get,
        (_, _, true, true) => EdgeTypeParser::to_numeric_unchecked,
        (_, _, false, true) => EdgeTypeParser::to_numeric,
    };
    let node_method = match (nodes.is_empty(), correct, numeric_edge_list_node_ids) {
        (true, true, false) => EdgeNodeNamesParser::parse_strings_unchecked,
        (true, false, false) => EdgeNodeNamesParser::parse_strings,
        (false, true, false) => EdgeNodeNamesParser::get_unchecked,
        (false, false, false) => EdgeNodeNamesParser::get,
        (_, true, true) => EdgeNodeNamesParser::to_numeric_unchecked,
        (_, false, true) => EdgeNodeNamesParser::to_numeric,
    };

    let mut edge_types_vocabulary = edge_types_vocabulary.unwrap_or(Vocabulary::new());

    // Here we handle the collection of the iterator
    // in a way to collect only non-None values and hence avoid
    // potentially a huge amount of allocations.
    let (edges, edge_type_ids, weights) = match (edges_iterators, has_edge_types, has_edge_weights)
    {
        (None, _, _) => {
            // Here likely needs to simply return None
        }
        (Some(eis), true, true) => {
            // Building the edge list
            let (edges, edge_type_ids, weights) = create_edge_list!(
                eis,
                nodes,
                node_method,
                edge_types_vocabulary,
                edge_types_method,
                (edge_type, weight),
                (edge_types, weights),
                (None, f64::NAN),
                directed,
                complete,
                duplicates
            );
            // Return the computed values
            (edges, Some(edge_type_ids), Some(weights))
        }
        (Some(eis), true, false) => {
            // Building the edge list
            let (edges, edge_type_ids) = create_edge_list!(
                eis,
                nodes,
                node_method,
                edge_types_vocabulary,
                edge_types_method,
                (edge_type),
                (edge_types),
                (None),
                directed,
                complete,
                duplicates
            );
            // Return the computed values
            (edges, Some(edge_type_ids), None)
        }
        (Some(eis), false, true) => {
            // Building the edge list
            let (edges, weights) = create_edge_list!(
                eis,
                nodes,
                node_method,
                edge_types_vocabulary,
                edge_types_method,
                (weight),
                (weights),
                (f64::NAN),
                directed,
                complete,
                duplicates
            );
            // Return the computed values
            (edges, None, Some(weights))
        }
        (Some(eis), false, false) => {
            // Building the edge list
            let (edges,) = create_edge_list!(
                eis,
                nodes,
                node_method,
                edge_types_vocabulary,
                edge_types_method,
                (),
                (),
                (),
                directed,
                complete,
                duplicates
            );
            // Return the computed values
            (edges, None, None)
        }
    };

    Ok((
        nodes,
        edges,
        EdgeTypeVocabulary::from_option_structs(edge_type_ids, Some(edge_types_vocabulary)),
        weights,
    ))
}