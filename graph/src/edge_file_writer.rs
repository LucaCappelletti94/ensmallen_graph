#[warn(unused_macros)]
use super::*;

/// Structure that saves the parameters specific to writing and reading a nodes csv file.
///
/// # Attributes
pub struct EdgeFileWriter {
    pub(crate) parameters: CSVFileWriter,
    pub(crate) sources_column: String,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column: String,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column: String,
    pub(crate) edge_types_column_number: usize,
    pub(crate) weights_column: String,
    pub(crate) weights_column_number: usize
}

impl EdgeFileWriter {
    /// Return new EdgeFileWriter object.
    ///
    /// # Arguments
    ///
    /// * parameters: CSVFileParameters - Path where to store/load the file.
    ///
    pub fn new(parameters: CSVFileWriter) -> EdgeFileWriter {
        EdgeFileWriter {
            parameters,
            sources_column: "subject".to_string(),
            sources_column_number: 0,
            destinations_column: "object".to_string(),
            destinations_column_number: 1,
            edge_types_column: "label".to_string(),
            edge_types_column_number: 2,
            weights_column: "weight".to_string(),
            weights_column_number: 3
        }
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column(mut self, sources_column: Option<String>) -> EdgeFileWriter {
        if let Some(column) = sources_column {
            self.sources_column = column;
        }
        self
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column_number: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column_number(
        mut self,
        sources_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = sources_column_number {
            self.sources_column_number = column_number;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destinations_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column(
        mut self,
        destinations_column: Option<String>,
    ) -> EdgeFileWriter {
        if let Some(column) = destinations_column {
            self.destinations_column = column;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destinations_column_number: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column_number(
        mut self,
        destinations_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = destinations_column_number {
            self.destinations_column_number = column_number;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * edge_types_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column(mut self, edge_type_column: Option<String>) -> EdgeFileWriter {
        if let Some(column) = edge_type_column {
            self.edge_types_column = column;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * edge_types_column_number: Option<usize> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column_number(
        mut self,
        edge_type_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = edge_type_column_number {
            self.edge_types_column_number = column_number;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * weights_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_weights_column(mut self, weights_column: Option<String>) -> EdgeFileWriter {
        if let Some(column) = weights_column {
            self.weights_column = column;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * weights_column_number: Option<usize> - The node types column to use for the file.
    ///
    pub fn set_weights_column_number(
        mut self,
        weights_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = weights_column_number {
            self.weights_column_number = column_number;
        }
        self
    }

    /// Write edge file.
    ///  
    pub(crate) fn write_edge_file(
        &self,
        sources: &Vec<NodeT>,
        destinations: &Vec<NodeT>,
        nodes_reverse_mapping: &Vec<String>,
        edge_type_reverse_mapping: &Option<Vec<String>>,
        edge_types: &Option<Vec<EdgeTypeT>>,
        weights: &Option<Vec<WeightT>>,
    ) -> Result<(), String> {
        // build the header
        let mut header = vec![
            (self.sources_column, self.sources_column_number),
            (self.destinations_column, self.destinations_column_number),
        ];

        if edge_types.is_some() {
            header.push((self.edge_types_column, self.edge_types_column_number));
        }

        if weights.is_some() {
            header.push((self.weights_column, self.weights_column_number));
        }

        let number_of_columns = 1 + header.iter().map(|(_, i)| i).max().unwrap();

        self.parameters.write_lines(
            sources.len() as u64,
            compose_lines(number_of_columns, header),
            (0..sources.len()).into_iter().map(|index| {
                let mut line = vec![
                    (
                        nodes_reverse_mapping[sources[index]],
                        self.sources_column_number,
                    ),
                    (
                        nodes_reverse_mapping[destinations[index]],
                        self.destinations_column_number,
                    ),
                ];

                if let Some(ets) = edge_types {
                    if let Some(etrm) = edge_type_reverse_mapping {
                        line.push((etrm[ets[index] as usize], self.edge_types_column_number));
                    }
                }

                if let Some(w) = weights {
                    line.push((
                        w[index].to_string(),
                        self.weights_column_number,
                    ));
                }

                compose_lines(number_of_columns, line)
            }),
        )
    }
}
