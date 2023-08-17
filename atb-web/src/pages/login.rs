use leptos::*;
use minecraft_msa_auth::MinecraftAuthorizationFlow;
use minecraft_msa_auth::MinecraftAccessToken;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse,
    TokenUrl, ClientSecret
};



#[component]
pub fn Login() -> View {
    view! {
        <div>
            <h1>
                Login
            </h1>
        </div>
    }
}


