use {leptos::*, leptos_router::*};

#[derive(Params, PartialEq, Clone, Debug, Default)]
struct QueryParams {
    code: Option<String>,
    state: Option<String>,
}

#[component]
pub fn AuthCallback() -> impl IntoView {
    let q = use_query::<QueryParams>();

    let q = q.get().unwrap_or_default();

    let q = format!("{q:?}");

    view! {
        <h1>"AuthCallback"</h1>
        // <div>Code: {x().0}</div>
        // <div>State: {x().1}</div>
        <div>{q}</div>
    }
}
