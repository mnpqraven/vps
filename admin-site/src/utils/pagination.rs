use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use proto_types::{common::db::Pagination, impls::DefaultState};

pub struct PaginationState {
    pub pagination: Signal<Pagination>,
    pub prev_params: Signal<String>,
    pub next_params: Signal<String>,
}
pub fn use_pagination() -> PaginationState {
    let query = use_query_map();
    let default = Pagination::default_state();
    let pagination = Signal::derive(move || {
        let page_index = query
            .read()
            .get("index")
            .map(|e| e.parse::<i32>().unwrap_or(default.page_index))
            .unwrap_or(default.page_index);
        let page_size = query
            .read()
            .get("size")
            .map(|e| e.parse::<i32>().unwrap_or(default.page_size))
            .unwrap_or(default.page_size);

        Pagination {
            page_index,
            page_size,
        }
    });

    let prev_params = Signal::derive(move || {
        let i = pagination.get().page_index - 1;
        match i > 0 {
            true => format!("index={i}"),
            false => format!("index=0"),
        }
    });
    let next_params = Signal::derive(move || {
        let i = pagination.get().page_index + 1;
        format!("index={i}")
    });

    PaginationState {
        pagination,
        prev_params,
        next_params,
    }
}
