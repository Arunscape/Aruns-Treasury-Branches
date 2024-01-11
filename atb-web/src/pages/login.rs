use leptos::*;
use minecraft_msa_auth::MinecraftAccessToken;
use minecraft_msa_auth::MinecraftAuthorizationFlow;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use leptos_router::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div>
            <h1>Login</h1>

            <p>In the future I hope to have some sort of passkey authentication here.</p>
            <p>For now, just follow the instructions at <A href="/signup">Signup</A></p>
        </div>
    }
}
