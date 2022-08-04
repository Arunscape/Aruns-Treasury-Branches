use dotenv::dotenv;
use lazy_static::lazy_static;
use tide::prelude::*;

use std::env;
use std::sync::{Arc, RwLock};

use juniper::{http::graphiql, http::GraphQLRequest, RootNode};
use tide::{http::mime, Body, Redirect, Request, Response, Server, StatusCode};

pub type Schema = RootNode<'static, QueryRoot, juniper::EmptyMutation<State>, juniper::EmptySubscription<State>>;

lazy_static! {
    static ref SCHEMA: Schema = Schema::new(QueryRoot {}, juniper::EmptyMutation::new(), juniper::EmptySubscription::new());
}

#[derive(Clone)]
pub struct State {}
impl juniper::Context for State {}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let address = env::var("ADDRESS").unwrap();
    let port = env::var("PORT").unwrap();
    tide::log::start();
    let mut app = Server::with_state(State {});
    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphql").post(handle_graphql);
    app.at("/graphiql").get(handle_graphiql);
    app.listen(format!("{}:{}", address, port))
        .await?;
    Ok(())
}

async fn handle_graphql(mut request: Request<State>) -> tide::Result {
    let query: GraphQLRequest = request.body_json().await?;
    let response = query.execute(&SCHEMA, request.state()).await;
    let status = if response.is_ok() {
        StatusCode::Ok
    } else {
        StatusCode::BadRequest
    };

    Ok(Response::builder(status)
        .body(Body::from_json(&response)?)
        .build())
}

async fn handle_graphiql(_: Request<State>) -> tide::Result<impl Into<Response>> {
    let address = env::var("ADDRESS").unwrap();
    let port = env::var("PORT").unwrap();
    Ok(Response::builder(200)
        .body(graphiql::graphiql_source(
            "/graphql",
            Some(&format!(
                "ws://{}:{}/subscriptions",
                address,
                port
            )),
        ))
        .content_type(mime::HTML))
}

pub struct QueryRoot;
#[juniper::graphql_object(Context=State)]
impl QueryRoot {
    fn idk(context: &State) -> i32 {
        69
    }
}

//pub struct MutationRoot;
//#[juniper::graphql_object(Context=State)]
//impl MutationRoot {}
//
//pub struct SubscriptionRoot;
//#[juniper::graphql_object(Context=State)]
//impl SubscriptionRoot {}
