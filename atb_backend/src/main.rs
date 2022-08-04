use tide::prelude::*;
#[macro_use]
extern crate dotenv_codegen;

use std::sync::{Arc, RwLock};

use juniper::{http::graphiql, http::GraphQLRequest, RootNode};
use tide::{http::mime, Body, Redirect, Request, Response, Server, StatusCode};


#[derive(Clone)]
struct State{}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    tide::log::start();
    let mut app = Server::with_state(State {
        
    });
    app.at("/").get(Redirect::permanent("/graphiql"));
    //app.at("/graphql").post(handle_graphql);
    //app.at("/graphiql").get(handle_graphiql);
    app.listen(format!("{}:{}", dotenv!("ADDRESS"), dotenv!("PORT"))).await?;
    Ok(())
}

