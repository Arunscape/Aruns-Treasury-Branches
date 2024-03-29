//use webauthn_rs_proto::*;

// Other imports needed to make the SPA (single page application) work.
use gloo::console;
//use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Request, RequestInit, RequestMode, Response, Window};
use {
    crate::components::SignupButton,
    leptos::*,
    std::{error::Error, fmt},
    wasm_bindgen::{prelude::*, JsCast},
};

// https://github.com/kanidm/webauthn-rs/blob/master/tutorial/wasm/src/lib.rs#L399

#[component]
pub fn Signup() -> impl IntoView {
    view! {
        <>
            <h1>How to sign up</h1>
            <ol class="pl-10 list-decimal hover:list-disc">
                <li>Open minecraft and login to mc.arun.gg</li>
                <li>type in /atb login</li>
                <li>open the link in your browser</li>
                <li>
                    optionally, register a passkey so you can login without first opening minecraft
                </li>
            </ol>
        </>
    }
}
