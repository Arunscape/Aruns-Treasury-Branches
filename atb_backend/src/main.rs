use dotenv::dotenv;
use lazy_static::lazy_static;
use tide::prelude::*;

use std::env;

use juniper::{http::graphiql, http::GraphQLRequest, RootNode};
use tide::{http::mime, Body, Redirect, Request, Response, Server, StatusCode};

pub type Schema = RootNode<
    'static,
    QueryRoot,
    juniper::EmptyMutation<Context>,
    juniper::EmptySubscription<Context>,
>;

lazy_static! {
    static ref SCHEMA: Schema = Schema::new(
        QueryRoot {},
        juniper::EmptyMutation::new(),
        juniper::EmptySubscription::new()
    );
    static ref ADDRESS: String = env::var("ADDRESS").unwrap();
    static ref PORT: String = env::var("PORT").unwrap();
}

#[derive(Clone)]
pub struct Context {}
impl juniper::Context for Context {}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tide::log::start();
    let mut app = Server::with_state(Context {});
    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphql").post(handle_graphql);
    app.at("/graphiql").get(handle_graphiql);
    app.listen(format!("{}:{}", *ADDRESS, *PORT)).await?;
    Ok(())
}

async fn handle_graphql(mut request: Request<Context>) -> tide::Result {
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

async fn handle_graphiql(_: Request<Context>) -> tide::Result<impl Into<Response>> {
    Ok(Response::builder(200)
        .body(graphiql::graphiql_source(
            "/graphql",
            Some(&format!("ws://{}:{}/subscriptions", *ADDRESS, *PORT)),
        ))
        .content_type(mime::HTML))
}

pub struct QueryRoot;
#[juniper::graphql_object(Context=Context)]
impl QueryRoot {
    fn idk(context: &Context) -> i32 {
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
