#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use atb_types::User;
    use atb_web::apiroutes::get_session_cookie;
    use atb_web::app::*;
    use atb_web::fileserv::file_and_error_handler;
    use axum::{
        routing::{get, post},
        Router,
    };
    use axum_session::{
        DatabasePool, Session, SessionConfig, SessionLayer, SessionPgPool, SessionStore,
    };
    use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::{
        postgres::{PgConnectOptions, PgPoolOptions},
        ConnectOptions, PgPool,
    };

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

    // https://github.com/leptos-rs/leptos/blob/main/examples/session_auth_axum/src/auth.rs

    let session_config = SessionConfig::default().with_table_name("axum_sessions");
    let auth_config = AuthConfig::<i64>::default();

    // TODO appstate goes here
    //    https://github.com/leptos-rs/leptos/blob/main/examples/session_auth_axum/src/main.rs#L24C3-L24C3
    //let session_store = SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config);

    //session_store.initiate().await.expect("couldn't init session store");

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/api/*fn_name", get(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .route("/api/get_session_cookie", get(get_session_cookie))
        //.layer(SessionLayer::new(session_store))
        //.layer(AuthSessionLayer::<User, i64, SessionPgPool, PgPool>::new(Some(pool)).with_config(auth_config))
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
