use std::ops::{Index, IndexMut};

use crate::{Alignment, Content, Dictionary, Integer, Or, Relative, TypedItem, types::{content::math::generic::Delim, generic::TypedArray}};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Matrix<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub delim: Option<Delim<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<TypedItem<Alignment>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub augment: Option<Or<Integer, Dictionary<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<TypedItem<Relative>>,
    #[serde(rename = "row-gap", skip_serializing_if = "Option::is_none")]
    pub row_gap: Option<TypedItem<Relative>>,
    #[serde(rename = "column-gap", skip_serializing_if = "Option::is_none")]
    pub column_gap: Option<TypedItem<Relative>>,
    #[serde(borrow)]
    pub rows: TypedArray<TypedArray<Content<'a>>>,
}

crate::impl_all_content!(Matrix<'a>.Math, "math.mat");

impl<'a> Matrix<'a> {
    pub fn rows(&self) -> usize {
        self.rows.len()
    }

    pub fn columns(&self) -> Result<usize, std::string::String> {
        if self.rows.is_empty() {
            Ok(0)
        } else {
            let len = self.rows[0].len();
            for row in &*self.rows {
                if row.len() != len {
                    return Err("Inconsistent row lengths in matrix".into());
                }
            }
            Ok(len)
        }
    }

    pub fn with_dimensions(rows: usize, columns: usize) -> Self {
        let row_array = vec![Content::default(); columns].into();
        let rows_array = vec![row_array; rows].into();
        Matrix {
            delim: None,
            align: None,
            augment: None,
            gap: None,
            row_gap: None,
            column_gap: None,
            rows: rows_array,
        }
    }
}

impl<'a> Index<(usize, usize)> for Matrix<'a> {
    type Output = Content<'a>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.rows[index.0][index.1]
    }
}

impl<'a> IndexMut<(usize, usize)> for Matrix<'a> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.rows[index.0][index.1]
    }
}