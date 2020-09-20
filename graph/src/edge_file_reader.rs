use super::*;

fn parse_edge_weight(weight: Option<String>) -> Result<Option<WeightT>, String> {
    match weight {
        None => Ok(None),
        Some(w) => match w.parse::<WeightT>() {
            Ok(val) => match val.is_finite() && val > 0.0 {
                true => Ok(Some(val)),
                false => Err(format!(
                    "The value {} parsed as a weight as {} is either infinite or NaN or Zero.",
                    w, val
                )),
            },
            Err(_) => Err(format!("Cannot parse weight {} as a float.", w)),
        },
    }
}

/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
pub struct EdgeFileReader {
    pub(crate) reader: CSVFileReader,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column_number: Option<usize>,
    pub(crate) default_edge_type: Option<String>,
    pub(crate) weights_column_number: Option<usize>,
    pub(crate) default_weight: Option<WeightT>,
    pub(crate) skip_self_loops: bool,
}

impl EdgeFileReader {
    /// Return new EdgeFileReader object.
    ///
    /// # Arguments
    ///
    /// * reader: CSVFilereader - Path where to store/load the file.
    ///
    pub fn new(path: String) -> Result<EdgeFileReader, String> {
        Ok(EdgeFileReader {
            reader: CSVFileReader::new(path)?,
            sources_column_number: 0,
            destinations_column_number: 1,
            edge_types_column_number: None,
            default_edge_type: None,
            weights_column_number: None,
            default_weight: None,
            skip_self_loops: false,
        })
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column(
        mut self,
        sources_column: Option<String>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = sources_column {
            self.sources_column_number = self.reader.get_column_number(column)?;
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column_number: Option<usize> - The sources column number to use for the file.
    ///
    pub fn set_sources_column_number(mut self, sources_column_number: Option<usize>) -> Result<EdgeFileReader, String> {
        if let Some(column) = sources_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if column >= expected_elements {
                return Err(format!(
                    concat!(
                        "The source column number passed was {} but",
                        "the first parsable line has {} values."
                    ),
                    column,
                    expected_elements
                ));
            }
            self.sources_column_number = column;
        }
        Ok(self)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destination_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column(
        mut self,
        destinations_column: Option<String>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = destinations_column {
            self.destinations_column_number = self.reader.get_column_number(column)?;
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * destinations_column_number: Option<usize> - The destinations column number to use for the file.
    ///
    pub fn set_destinations_column_number(mut self, destinations_column_number: Option<usize>) -> Result<EdgeFileReader, String> {
        if let Some(column) = destinations_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if column >= expected_elements {
                return Err(format!(
                    concat!(
                        "The destinations column number passed was {} but",
                        "the first parsable line has {} values."
                    ),
                    column,
                    expected_elements
                ));
            }
            self.destinations_column_number = column;
        }
        Ok(self)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destination_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column(
        mut self,
        edge_type_column: Option<String>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = edge_type_column {
            self.edge_types_column_number = Some(self.reader.get_column_number(column)?);
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * edge_types_column_number: Option<usize> - The edge_types column number to use for the file.
    ///
    pub fn set_edge_types_column_number(mut self, edge_types_column_number: Option<usize>) -> Result<EdgeFileReader, String> {
        if let Some(etcn) = &edge_types_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if *etcn >= expected_elements {
                return Err(format!(
                    concat!(
                        "The edge types column number passed was {} but",
                        "the first parsable line has {} values."
                    ),
                    etcn,
                    expected_elements
                ));
            }
        }
        self.edge_types_column_number = edge_types_column_number;
        Ok(self)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destination_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_weights_column(
        mut self,
        weights_column: Option<String>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = weights_column {
            self.weights_column_number = Some(self.reader.get_column_number(column)?);
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * weights_column_number: Option<usize> - The weights column number to use for the file.
    ///
    pub fn set_weights_column_number(mut self, weights_column_number: Option<usize>) -> Result<EdgeFileReader, String> {
        if let Some(wcn) = &weights_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if *wcn >= expected_elements {
                return Err(format!(
                    concat!(
                        "The weights column number passed was {} but",
                        "the first parsable line has {} values."
                    ),
                    wcn,
                    expected_elements
                ));
            }
        }
        self.weights_column_number = weights_column_number;
        Ok(self)
    }

    /// Set the default default_weight.
    ///
    /// # Arguments
    ///
    /// * default_weight: Option<WeightT> - The default_weight to use when default_weight is missing.
    ///
    pub fn set_default_weight(mut self, default_weight: Option<WeightT>) -> EdgeFileReader {
        self.default_weight = default_weight;
        self
    }

    /// Set the default edge type.
    ///
    /// # Arguments
    ///
    /// * default_edge_type: Option<String> - The edge type to use when edge type is missing.
    ///
    pub fn set_default_edge_type(mut self, default_edge_type: Option<String>) -> EdgeFileReader {
        self.default_edge_type = default_edge_type;
        self
    }

    /// Set if the reader should ignore or not duplicated edges.
    ///
    /// # Arguments
    ///
    /// * skip_self_loops: Option<bool> - if the reader should ignore or not duplicated edges.
    ///
    pub fn set_skip_self_loops(mut self, skip_self_loops: Option<bool>) -> EdgeFileReader {
        if let Some(i) = skip_self_loops {
            self.skip_self_loops = i;
        }
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - Wethever to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> EdgeFileReader {
        if let Some(v) = verbose {
            self.reader.verbose = v;
        }
        self
    }

    /// Set the ignore_duplicates.
    ///
    /// # Arguments
    ///
    /// * ignore_duplicates: Option<bool> - Wethever to ignore detected duplicates or raise exception.
    ///
    pub fn set_ignore_duplicates(mut self, ignore_duplicates: Option<bool>) -> EdgeFileReader {
        if let Some(v) = ignore_duplicates {
            self.reader.ignore_duplicates = v;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<String>) -> EdgeFileReader {
        if let Some(v) = separator {
            self.reader.separator = v;
        }
        self
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - Wethever to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> EdgeFileReader {
        if let Some(v) = header {
            self.reader.header = v;
        }
        self
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    ///
    /// * rows_to_skip: Option<bool> - Wethever to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> EdgeFileReader {
        if let Some(v) = rows_to_skip {
            self.reader.rows_to_skip = v;
        }
        self
    }

    /// Parse a single line (vecotr of strings already splitted)
    /// # Arguments
    ///
    /// * vals: Vec<String> - Vector of the values of the line to be parsed
    fn parse_edge_line(
        &self,
        vals: Vec<String>,
    ) -> Result<(String, String, Option<String>, Option<WeightT>), String> {
        // exctract the values
        let source_node_name = vals[self.sources_column_number].to_owned();
        let destination_node_name = vals[self.destinations_column_number].to_owned();
        // extract the edge type if present
        let edge_type: Option<String> = match self.edge_types_column_number {
            None => Ok(None),
            Some(idx) => {
                let curr = vals[idx].to_owned();
                if !curr.is_empty() {
                    Ok(Some(curr))
                } else if let Some(def) = &self.default_edge_type {
                    Ok(Some(def.clone()))
                } else {
                    Err(format!(
                        concat!(
                            "Found empty edge type but no default edge ",
                            "type to use was provided.",
                            "The source node name is {source_node_name}.\n",
                            "The destination node name is {destination_node_name}.\n",
                            "The path of the document was {path}.\n"
                        ),
                        source_node_name = source_node_name,
                        destination_node_name = destination_node_name,
                        path = self.reader.path
                    ))
                }
            }
        }?;
        // extract the weights
        let edge_weight = match self.weights_column_number {
            None => Ok(None),
            Some(idx) => {
                let curr = vals[idx].to_owned();
                if !curr.is_empty() {
                    match parse_edge_weight(Some(curr)) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(e),
                    }
                } else if let Some(def) = &self.default_weight {
                    Ok(Some(*def))
                } else {
                    Err(format!(
                        concat!(
                            "Found empty weight but no default weight ",
                            "to use was provided.",
                            "The source node name is {source_node_name}.\n",
                            "The destination node name is {destination_node_name}.\n",
                            "The path of the document was {path}.\n"
                        ),
                        source_node_name = source_node_name,
                        destination_node_name = destination_node_name,
                        path = self.reader.path
                    ))
                }
            }
        }?;

        Ok((
            source_node_name,
            destination_node_name,
            edge_type,
            edge_weight,
        ))
    }

    /// Return iterator of rows of the edge file.
    pub fn read_lines(
        &self,
    ) -> Result<
        impl Iterator<Item = Result<(String, String, Option<String>, Option<WeightT>), String>> + '_,
        String,
    > {
        let expected_elements = self.reader.get_elements_per_line()?;
        if self.sources_column_number >= expected_elements {
            return Err(format!(
                concat!(
                    "The sources column number passed was {} but",
                    "the first parsable line has {} values."
                ),
                self.sources_column_number,
                expected_elements
            )); 
        }
        if self.destinations_column_number >= expected_elements {
            return Err(format!(
                concat!(
                    "The destinations column number passed was {} but",
                    "the first parsable line has {} values."
                ),
                self.destinations_column_number,
                expected_elements
            ));
        }
        Ok(self
            .reader
            .read_lines()?
            .map(move |values| match values {
                Ok(vals) => self.parse_edge_line(vals),
                Err(e) => Err(e),
            }))
    }
}
