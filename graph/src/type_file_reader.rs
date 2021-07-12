use rayon::iter::ParallelIterator;

use super::*;
/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
#[derive(Clone)]
pub struct TypeFileReader {
    pub(crate) reader: CSVFileReader,
    pub(crate) type_column_number: usize,
    pub(crate) numeric_type_ids: bool,
}

impl TypeFileReader {
    /// Return new TypeFileReader object.
    ///
    /// # Arguments
    /// * reader: CSVFilereader - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> Result<TypeFileReader> {
        Ok(TypeFileReader {
            reader: CSVFileReader::new(path, "type list".to_owned())?,
            type_column_number: 0,
            numeric_type_ids: false,
        })
    }

    /// Set the column of the type nodes.
    ///
    /// # Arguments
    /// * types_column: Option<String> - The type nodes column to use for the file.
    ///
    pub fn set_type_column<S: Into<String>>(
        mut self,
        type_column: Option<S>,
    ) -> Result<TypeFileReader> {
        if let Some(column) = sources_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given column name is empty.".to_owned());
            }

            match self.reader.get_column_number(column) {
                Ok(ecn) => {
                    self = self.set_sources_column_number(Some(ecn))?;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(self)
    }

    /// Set the column number of the types.
    ///
    /// # Arguments
    /// * type_column_number: Option<usize> - The type column number to use for the file.
    ///
    pub fn set_type_column_number(
        mut self,
        type_column_number: Option<usize>,
    ) -> Result<TypeFileReader> {
        if let Some(column) = type_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if column >= expected_elements {
                return Err(format!(
                    concat!(
                        "The type column number passed was {} but ",
                        "the first parsable line has {} values."
                    ),
                    column, expected_elements
                ));
            }
            self.type_column_number = column;
        }
        Ok(self)
    }

    /// Set whether the CSV is expected to be well written.
    ///
    /// # Arguments
    /// * csv_is_correct: Option<bool> - Whether you pinky swear the edge list is correct.
    ///
    pub fn set_csv_is_correct(mut self, csv_is_correct: Option<bool>) -> TypeFileReader {
        if let Some(cic) = csv_is_correct {
            self.reader.csv_is_correct = cic;
        }
        self
    }

    /// Set the comment symbol to use to skip the lines.
    ///
    /// # Arguments
    /// * comment_symbol: Option<String> - if the reader should ignore or not duplicated edges.
    ///
    pub fn set_comment_symbol(mut self, comment_symbol: Option<String>) -> Result<TypeFileReader> {
        if let Some(cs) = comment_symbol {
            if cs.is_empty() {
                return Err("The given comment symbol is empty.".to_string());
            }
            self.reader.comment_symbol = Some(cs);
        }
        Ok(self)
    }

    /// Set the verbose.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> TypeFileReader {
        if let Some(v) = verbose {
            self.reader.verbose = v;
        }
        self
    }

    ///
    /// * numeric_id: Option<bool> - Whether to convert numeric Ids to Node Id.
    ///
    pub fn set_numeric_type_ids(
        mut self,
        numeric_type_ids: Option<bool>,
    ) -> TypeFileReader {
        if let Some(neti) = numeric_type_ids {
            self.numeric_type_ids = neti;
        }
        self
    }

    /// Set the ignore_duplicates.
    ///
    /// # Arguments
    /// * ignore_duplicates: Option<bool> - Whether to ignore detected duplicates or raise exception.
    ///
    pub fn set_ignore_duplicates(mut self, ignore_duplicates: Option<bool>) -> TypeFileReader {
        if let Some(v) = ignore_duplicates {
            self.reader.ignore_duplicates = v;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator<S: Into<String>>(
        mut self,
        separator: Option<S>,
    ) -> Result<TypeFileReader> {
        if let Some(sep) = separator {
            let sep = sep.into();
            if sep.is_empty() {
                return Err("The separator cannot be empty.".to_owned());
            }
            self.reader.separator = sep;
        }
        Ok(self)
    }

    /// Set the header.
    ///
    /// # Arguments
    /// * header: Option<bool> - Whether to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> TypeFileReader {
        if let Some(v) = header {
            self.reader.header = v;
        }
        self
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    /// * rows_to_skip: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> TypeFileReader {
        if let Some(v) = rows_to_skip {
            self.reader.rows_to_skip = v;
        }
        self
    }

    /// Set the maximum number of rows to load from the file
    ///
    /// # Arguments
    /// * max_rows_number: Option<u64> - The edge type to use when edge type is missing.
    ///
    pub fn set_max_rows_number(mut self, max_rows_number: Option<u64>) -> TypeFileReader {
        self.reader.max_rows_number = max_rows_number;
        self
    }

    /// Parse a single line (vecotr of strings already splitted)
    /// # Arguments
    /// * vals: Vec<String> - Vector of the values of the line to be parsed
    fn parse_type_line(&self, vals: Vec<Option<String>>) -> Result<String> {
        // extract the type name
        Ok(vals[self.type_column_number].to_owned().unwrap())
    }

    /// Return iterator of rows of the edge file.
    pub fn read_lines(
        &self,
    ) -> Result<impl ParallelIterator<Item = Result<(usize, String)>> + '_> {
        let expected_elements = self.reader.get_elements_per_line()?;
        Ok(self
            .reader
            .read_lines()?
            .map(move |line| match line {
                Ok((line_number, vals)) => Ok((line_number, self.parse_edge_line(vals)?)),
                Err(e) => Err(e),
            }))
    }
}