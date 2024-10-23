use {
    crate::serverfns::login::RegisterPasskey,
    gloo::console,
    leptos::*,
    leptos_router::*,
    std::{error::Error, fmt},
    wasm_bindgen::{prelude::*, JsCast},
    wasm_bindgen_futures::JsFuture,
    web_sys::{Document, Request, RequestInit, RequestMode, Response, Window},
    webauthn_rs_proto::*,
};

#[component]
pub fn Login() -> impl IntoView {
    let register_action = create_server_action::<RegisterPasskey>();
    let login_action = create_server_action::<RegisterPasskey>();

    view! {
        <div>
            <h1>"Login Page"</h1>

            <ActionForm action=register_action>

                <label>
                    "Sign up: enter your desired username" <input name="new_username" type="text" />
                </label>
                <button type="submit">{"Sign up"}</button>
            </ActionForm>

            <ActionForm action=login_action>
                <label>
                    "Login: Enter your already existing username here"
                    <input name="existing_username" type="text" />
                </label>
            </ActionForm>
        </div>
    }
}
