use leptos::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct ColumnDef<T> {
    header: String,
    render_fn: ViewFn<T>,
}

#[derive(Clone, Default)]
pub struct ColumnDefs<T>(pub Vec<ColumnDef<T>>);

/// like `<Show />` components from leptos
#[derive(Clone)]
struct ViewFn<T>(Arc<dyn Fn(&T) -> AnyView + Send + Sync + 'static>);

impl<T> ViewFn<T> {
    fn run(&self, t: &T) -> AnyView {
        (self.0)(t)
    }
}

impl<T> ColumnDefs<T> {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn col(
        mut self,
        header: impl ToString,
        render_fn: impl Fn(&T) -> AnyView + Send + Sync + 'static,
    ) -> Self {
        self.0.push(ColumnDef {
            header: header.to_string(),
            render_fn: ViewFn(Arc::new(render_fn)),
        });
        self
    }
}

#[component]
pub fn Table<T>(data: Vec<T>, column_defs: ColumnDefs<T>) -> impl IntoView {
    let th_header = column_defs
        .0
        .iter()
        .map(|def| {
            view! { <th>{def.header.clone()}</th> }
        })
        .collect_view();

    let rows = data
        .iter()
        .map(|data_row| {
            let cells = column_defs
                .0
                .iter()
                .map(|def| view! { <td>{def.render_fn.run(data_row)}</td> })
                .collect_view();
            view! { <tr>{cells}</tr> }
        })
        .collect_view();

    view! {
        <table class="border">
            <tbody>
                <tr class="border border-b">
                    {th_header}
                </tr>
                {rows}
            </tbody>
        </table>
    }
}
