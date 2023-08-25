use crate::components::status::McStatusComponent;
use leptos::*;

#[component]
pub fn Status() -> impl IntoView {
    view! {
        <h1>"Status"</h1>
        <McStatusComponent/>
    }
}
