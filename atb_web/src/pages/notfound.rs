use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.back());
    html! {
        <div>
            <h1>{ "AAH! That page does not exist" }</h1>
            <button {onclick}>{ "Go Back" }</button>
        </div>
    }
}