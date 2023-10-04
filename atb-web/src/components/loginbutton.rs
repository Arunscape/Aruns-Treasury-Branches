use crate::serverfns::login::*;
use leptos::*;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};

const DARKSVG: &'static str = "https://learn.microsoft.com/en-us/azure/active-directory/develop/media/howto-add-branding-in-apps/ms-symbollockup_signin_dark_short.svg";

const LIGHTSVG: &'static str = "https://learn.microsoft.com/en-us/azure/active-directory/develop/media/howto-add-branding-in-apps/ms-symbollockup_signin_light_short.svg";

const CLIENT_ID: &'static str = "b65a4f90-a35f-4345-a755-fa4c05c7a8d9";

const AUTH_URL: &'static str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";

const TOKEN_URL: &'static str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";

// todo make it an environment variable?
const REDIRECT_URI: &'static str = if cfg!(debug_assertions) {
    "http://localhost:3000/api/auth/callback/azure-ad"
} else {
    "http://atb.arun.gg/api/auth/callback/azure-ad"
};

#[component]
pub fn LoginButton() -> impl IntoView {
    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.into()),
        None,
        AuthUrl::new(AUTH_URL.into())?,
        Some(TokenUrl::new(TOKEN_URL.into())?),
    )
    .set_auth_type(AuthType::RequestBody)
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URI.into())?);

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("XboxLive.signin offline_access".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    let authorize_url = authorize_url.to_string();

    let v = view! {
        <a href=authorize_url>
            <img src=DARKSVG alt="Sign in with Microsoft"/>
        </a>
    };

    Ok::<_, ServerFnError>(v)
}
