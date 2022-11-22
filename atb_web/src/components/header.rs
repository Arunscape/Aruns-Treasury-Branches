use crate::Route;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use std::collections::HashMap;
use std::str::FromStr;
use yew::prelude::*;
use yew_router::prelude::*;
use uuid::Uuid;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
struct UserInfo {
    name: String,
    uuid: String,
}

#[function_component(Header)]
pub fn login() -> Html {
    let location = use_location().unwrap();

    let route: Option<Route> = location.route();

    let username = use_state(|| UserInfo{ name: "".to_string(), uuid: "".to_string() });
    {
        let username = username.clone();
        use_effect_with_deps(move |_| {
            let username = username.clone();
            wasm_bindgen_futures::spawn_local(async move {

                let uuid = "cdb5aee80f904fdda63ba16d38cd6b3b";
                let request_url = format!("https://api.mojang.com/users/profiles/minecraft/{}", uuid);
                let fetched_username: UserInfo = Request::get(&request_url)
                    .header("Access-Control-Allow-Origin", "*")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                username.set(fetched_username);
            });
            || ()
        }, ());
    }

    html! {
        <nav>
            <h1>{ "Header" }</h1>
            if route.is_some() {
                <p>{ format!("Current route: {:?}", route.unwrap()) }</p>
            }
            <p>{ format!("Username: {}", username.name) }</p>
        </nav>
    }
}
