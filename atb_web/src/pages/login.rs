use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use crate::Route;
use std::collections::HashMap;
use std::str::FromStr;

#[function_component(Login)]
pub fn login() -> Html {
    let location = use_location().unwrap();

    let query_params: HashMap<String, String> = location.query().unwrap_or_default();

    let token= query_params.get("token");

    if let Some(t) = token {
        LocalStorage::set("token", t).unwrap();
    }


    html! {
        <Redirect<Route> to={Route::Home}/>
    }
}