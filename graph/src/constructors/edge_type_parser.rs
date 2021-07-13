use super::*;

impl_struct_func!(EdgeTypeParser Vocabulary<EdgeTypeT>);

impl EdgeTypeParser {
    pub fn ignore<T, W>(
        &mut self,
        value: Result<(usize, (T, T, Option<String>, W))>,
    ) -> Result<(usize, (T, T, Option<EdgeTypeT>, W))> {
        let (line_number, (src, dst, _edge_type_name, weight)) = value?;
        Ok((line_number, (src, dst, None, weight)))
    }

    pub fn parse_strings<T, W>(
        &mut self,
        value: Result<(usize, (T, T, Option<String>, W))>,
    ) -> Result<(usize, (T, T, Option<EdgeTypeT>, W))> {
        let (line_number, (src, dst, edge_type_name, weight)) = value?;
        let vocabulary = self.get_mutable_write();
        Ok((
            line_number,
            (
                src,
                dst,
                Some(
                    vocabulary
                        .0
                        .insert(unsafe { edge_type_name.unwrap_unchecked() })?
                        .0,
                ),
                weight,
            ),
        ))
    }

    pub fn parse_strings_unchecked<T, W>(
        &mut self,
        value: Result<(usize, (T, T, Option<String>, W))>,
    ) -> Result<(usize, (T, T, Option<EdgeTypeT>, W))> {
        let (line_number, (src, dst, edge_type_name, weight)) = value?;
        let vocabulary = self.get_mutable_write();
        unsafe {
            Ok((
                line_number,
                (
                    src,
                    dst,
                    Some(
                        vocabulary
                            .0
                            .unchecked_insert(edge_type_name.unwrap_unchecked()),
                    ),
                    weight,
                ),
            ))
        }
    }

    pub fn get<T, W>(
        &mut self,
        value: Result<(usize, (T, T, Option<String>, W))>,
    ) -> Result<(usize, (T, T, Option<EdgeTypeT>, W))> {
        let (line_number, (src, dst, edge_type_name, weight)) = value?;
        let vocabulary = self.get_immutable();
        let edge_type_name = unsafe { &edge_type_name.unwrap_unchecked() };
        Ok((
            line_number,
            (
                src,
                dst,
                Some(match vocabulary.get(&edge_type_name) {
                    Some(et) => Ok(et),
                    None => Err(format!(
                        concat!(
                            "Found an unknown edge type while reading the edge list.\n",
                            "Specifically the unknown edge type is {:?}.\n",
                            "The list of the known edge types is {:#4?}"
                        ),
                        edge_type_name,
                        vocabulary.keys()
                    )),
                }?),
                weight,
            ),
        ))
    }

    pub fn get_unchecked<T, W>(
        &mut self,
        value: Result<(usize, (T, T, Option<String>, W))>,
    ) -> Result<(usize, (T, T, Option<EdgeTypeT>, W))> {
        let (line_number, (src, dst, edge_type_name, weight)) = value?;
        let vocabulary = self.get_immutable();
        Ok((
            line_number,
            (
                src,
                dst,
                vocabulary.get(&unsafe { edge_type_name.unwrap_unchecked() }),
                weight,
            ),
        ))
    }

    pub fn to_numeric<T, W>(
        &mut self,
        value: Result<(usize, (T, T, Option<String>, W))>,
    ) -> Result<(usize, (T, T, Option<EdgeTypeT>, W))> {
        let (line_number, (src, dst, edge_type_name, weight)) = value?;
        let vocabulary = self.get_immutable();
        let edge_type_id = match unsafe { edge_type_name.clone().unwrap_unchecked() }.parse::<EdgeTypeT>() {
            Ok(edge_type_id) => Ok::<_, String>(edge_type_id),
            Err(_) => Err::<_, String>(format!(
                concat!(
                    "The given edge type name {:?} ",
                    "cannot be parsed to an integer value."
                ),
                edge_type_name
            )),
        }?;
        if vocabulary.len() as EdgeTypeT <= edge_type_id {
            return Err(format!(
                concat!(
                    "The given edge type name {:?} ",
                    "has a value greater than the number ",
                    "of provided nodes {}."
                ),
                edge_type_id,
                vocabulary.len()
            ));
        }
        Ok((line_number, (src, dst, Some(edge_type_id), weight)))
    }

    pub fn to_numeric_unchecked<T, W>(
        &mut self,
        value: Result<(usize, (T, T, Option<String>, W))>,
    ) -> Result<(usize, (T, T, Option<EdgeTypeT>, W))> {
        let (line_number, (src, dst, edge_type_name, weight)) = value?;
        let vocabulary = self.get_immutable();
        unsafe {
            Ok((
                line_number,
                (
                    src,
                    dst,
                    Some(
                        unsafe { edge_type_name.unwrap_unchecked() }
                            .parse::<EdgeTypeT>()
                            .unwrap_unchecked(),
                    ),
                    weight,
                ),
            ))
        }
    }
}
