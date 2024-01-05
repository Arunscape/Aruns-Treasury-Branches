
use leptos::*;
use crate::components::LoginButton;
// https://github.com/kanidm/webauthn-rs/blob/master/tutorial/wasm/src/lib.rs#L399

#[component]
pub fn Signup() -> impl IntoView {
    view! {
        <div>
            <h1>Signup</h1>
            <LoginButton />
        </div>
    }
}
