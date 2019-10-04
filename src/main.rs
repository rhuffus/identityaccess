extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate env_logger;
extern crate juniper;
extern crate serde_json;

use std::io;
use std::sync::Arc;

use actix_web::{App, Error, HttpResponse, HttpServer, middleware, web};
use diesel::prelude::*;
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::infrastructure::graphql::schema::{create_schema, Schema};
use crate::infrastructure::postgres::connection::establish_connection;

mod domain;
mod infrastructure;

fn graphiql() -> HttpResponse {
	let html = graphiql_source("http://127.0.0.1:8080/graphql");
	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(html)
}

pub struct QueryContext {
	database: PgConnection
}

impl juniper::Context for QueryContext {}

fn graphql(
	st: web::Data<Arc<Schema>>,
	data: web::Json<GraphQLRequest>,
) -> impl Future<Item=HttpResponse, Error=Error> {
	web::block(move || {
		let context = QueryContext {
			database: establish_connection(),
		};
		let res = data.execute(&st, &context);
		Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
	})
		.map_err(Error::from)
		.and_then(|response| {
			Ok(HttpResponse::Ok()
				.content_type("application/json")
				.body(response))
		})
}

fn main() -> io::Result<()> {
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	// Create Juniper schema
	let schema = std::sync::Arc::new(create_schema());

	// Start http server
	HttpServer::new(move || {
		App::new()
			.data(schema.clone())
			.wrap(middleware::Logger::default())
			.service(web::resource("/graphql").route(web::post().to_async(graphql)))
			.service(web::resource("/graphiql").route(web::get().to(graphiql)))
	})
		.bind("localhost:8080")?
		.run()
}
