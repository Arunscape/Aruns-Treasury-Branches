use {
    leptos::*,
    leptos_router::*,
    //    oauth2::{
    //        basic::BasicClient, reqwest::async_http_client, AuthType, AuthUrl, AuthorizationCode,
    //        ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse,
    //        TokenUrl,
    //    },
};

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
