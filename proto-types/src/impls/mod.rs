use crate::common::db::Pagination;

pub trait ApiDefault {
    fn api_default() -> Self;
}

impl ApiDefault for Pagination {
    fn api_default() -> Self {
        Self {
            page_index: 0,
            page_size: 10,
        }
    }
}
