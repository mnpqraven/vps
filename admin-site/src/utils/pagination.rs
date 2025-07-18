use leptos::{attr::any_attribute::AnyAttribute, prelude::*};
use leptos_router::params::Params;
use leptos_router::{components::A, hooks::use_query};
use proto_types::{common::db::Pagination, impls::DefaultState};
use strum::{Display, EnumString};

use crate::ui::primitive::button::Button;

#[derive(Clone, Params, PartialEq)]
struct PaginationQuery {
    index: Option<i32>,
    size: Option<i32>,
    search: Option<String>,
}

impl From<&PaginationQuery> for Pagination {
    fn from(
        PaginationQuery {
            index,
            size,
            search,
        }: &PaginationQuery,
    ) -> Self {
        let default = Pagination::default_state();
        Self {
            page_index: index.unwrap_or(default.page_index),
            page_size: size.unwrap_or(default.page_size),
            search: search.clone().unwrap_or_default(),
        }
    }
}

pub struct PaginationState {
    pub pagination: Signal<Pagination>,
    pub prev_params: Signal<String>,
    pub next_params: Signal<String>,
}
pub fn use_pagination() -> PaginationState {
    let query = use_query::<PaginationQuery>();

    let default = Pagination::default_state();
    let pagination: Signal<Pagination> = Signal::derive(move || {
        query
            .read()
            .as_ref()
            .ok()
            .map(Into::into)
            .unwrap_or(default.clone())
    });

    let prev_params = Signal::derive(move || {
        let i = pagination.get().page_index - 1;
        match i > 0 {
            true => format!("index={i}"),
            false => String::new(),
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

#[derive(Debug, EnumString, Display, Clone)]
#[strum(ascii_case_insensitive)]
pub enum PaginationDirection {
    Prev,
    Next,
}
#[component]
pub fn PaginationButton(
    #[prop(into)] direction: Signal<PaginationDirection>,
    pagination: Signal<Pagination>,
    #[prop(optional)] attr: Vec<AnyAttribute>,
) -> impl IntoView {
    let params = Signal::derive(move || {
        let i = pagination.get().page_index;
        match direction.get() {
            PaginationDirection::Prev => match i > 0 {
                true => format!("index={}", i - 1),
                false => String::new(),
            },
            PaginationDirection::Next => format!("index={}", i + 1),
        }
    });

    view! {
        <A href=move || format!("?{}", params.get())>
            <Button {..attr}>{direction.with(ToString::to_string)}</Button>
        </A>
    }
}
