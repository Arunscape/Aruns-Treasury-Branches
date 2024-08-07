//https://github.com/DarrenBaldwin07/clerk-rs

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![feature(lazy_cell)]
#[cfg(feature = "ssr")]
use {
    atb_types::User,
    atb_web::{apiroutes::get_session_cookie, app::*, fileserv::file_and_error_handler},
    axum::{
        body::Body as AxumBody,
        extract::{FromRef, Path, State},
        http::Request,
        response::{IntoResponse, Response},
        routing::{get, post},
        Router,
    },
    axum_session::{
        DatabasePool, Session, SessionAnyPool, SessionConfig, SessionLayer, SessionStore,
    },
    axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication, HasPermission},
    http::HeaderMap,
    jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation},
    leptos::{LeptosOptions, *},
    leptos_axum::handle_server_fns_with_context,
    leptos_axum::{generate_route_list, LeptosRoutes},
    leptos_router::RouteListing,
    serde::{Deserialize, Serialize},
    sqlx::{
        any::{AnyConnectOptions, AnyPoolOptions},
        postgres::{PgConnectOptions, PgPoolOptions},
        AnyPool, ConnectOptions, PgPool,
    },
    std::net::SocketAddr,
    std::str::FromStr,
    std::sync::LazyLock,
    uuid::Uuid,
};

static DB_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost/postgres".into())
});

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    //let routes = generate_route_list(|| view! { <App/> }).await;
    let routes = generate_route_list(App);

    //sqlx::any::install_default_drivers();
    //let connect_opts = AnyConnectOptions::from_str(&DB_URL)?;
    let connect_opts = PgConnectOptions::from_str(&DB_URL)?;
    //connect_opts.log_statements(tracing::log::LevelFilter::Debug);

    //let pool = AnyPoolOptions::new()
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect_with(connect_opts)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
        routes: routes.clone(),
    };

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            post(server_fn_handler).get(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        //        .route("/api/get_session_cookie", get(get_session_cookie))
        //.layer(SessionLayer::new(session_store))
        //.layer(AuthSessionLayer::<User, i64, SessionPgPool, PgPool>::new(Some(pool)).with_config(auth_config))
        .route("/mcplugin/link_account", post(link_account))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(feature = "ssr")]
/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
//#[derive(FromRef, Debug, Clone)]
//pub struct AppState {
//    pub leptos_options: LeptosOptions,
//    pub pool: AnyPool,
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    //pub pool: AnyPool,
    pub pool: PgPool,
    pub routes: Vec<RouteListing>,
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}

#[cfg(feature = "ssr")]
async fn server_fn_handler(
    State(app_state): State<AppState>,
    _path: Path<String>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.pool.clone());
        },
        request,
    )
    .await
}

#[cfg(feature = "ssr")]
async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(app_state.pool.clone());
        },
        App,
    );
    handler(req).await.into_response()
}

#[cfg(feature = "ssr")]
async fn link_account(headers: HeaderMap) -> impl IntoResponse {
    dbg!(&headers);
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        uuid: Uuid,
        username: String,
    }
    let token = headers.get("authorization");
    dbg!(&token);
    let token = token.unwrap().to_str().unwrap();
    dbg!(&token);
    let key = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEA6tuVddasOcnOWutIMKGhctubzk6iZm6nc4zqi3gtC+g=
-----END PUBLIC KEY-----";
    let key = DecodingKey::from_ed_pem(key.as_ref()).unwrap();

    let mut validation = Validation::new(jsonwebtoken::Algorithm::EdDSA);
    validation.set_audience(&["atb.arun.gg"]);
    validation.set_issuer(&["mc.arun.gg"]);

    let claims = decode::<Claims>(&token, &key, &validation);

    dbg!(&claims);
}
