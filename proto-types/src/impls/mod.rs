use crate::common::db::Pagination;

pub trait DefaultState {
    fn default_state() -> Self;
}

impl DefaultState for Pagination {
    fn default_state() -> Self {
        Self {
            page_index: 0,
            page_size: 10,
            search: String::new(),
        }
    }
}
