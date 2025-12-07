use crate::common::db::ProtoPagination;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use utoipa::{IntoParams, ToSchema};

/// standard pagination data shape with defined default numbers (index 0, size 10)
/// instead of default 0 for both fields
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Pagination {
    pub page_index: i32,
    pub page_size: i32,
    pub search: Option<String>,
    pub all: bool,
}

// TODO: prost::IntoRequest impl

impl From<ProtoPagination> for Pagination {
    fn from(value: ProtoPagination) -> Self {
        Self {
            page_index: value.page_index.unwrap_or_default(),
            page_size: value.page_size.unwrap_or(10),
            search: value.search.clone(),
            all: value.all(),
        }
    }
}

impl From<Pagination> for ProtoPagination {
    fn from(value: Pagination) -> Self {
        Self {
            page_index: Some(value.page_index),
            page_size: Some(value.page_size),
            search: value.search,
            all: Some(value.all),
        }
    }
}

pub trait Paged {
    /// if `all: None` is passed, the list is returned as-is
    /// NOTE: can optimize in the future, as this function uses `to_vec()` and
    /// therefore always copies
    fn paginate_by<T>(self, pg: Pagination) -> (Vec<T>, PaginationMeta)
    where
        Self: Sized,
        Self: Deref<Target = [T]>,
        T: Clone,
    {
        if pg.all {
            let meta = PaginationMeta {
                page_index: 0,
                page_size: self.len() as i32,
                total: self.len() as i32,
                has_next: false,
            };
            (self.to_vec(), meta)
        } else {
            let (size, index) = (pg.page_size, pg.page_index);
            let data = self
                .chunks(size as usize)
                .nth(index as usize)
                .unwrap_or_default()
                .to_vec();

            let meta = PaginationMeta {
                page_index: index,
                page_size: size,
                total: self.len() as i32,
                has_next: (index + 1) * size <= self.len() as i32,
            };
            (data, meta)
        }
    }
}

impl<T> Paged for Vec<T> {}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    /// 0-based pagination indexing
    pub page_index: i32,
    pub page_size: i32,
    pub total: i32,
    pub has_next: bool,
}
