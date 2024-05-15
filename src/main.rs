// region:    --- Modules

mod ctx;
mod web;

use async_graphql::http::GraphiQLSource;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing::debug;
use tracing_subscriber::EnvFilter;

// endregion: --- Modules

pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .without_time() // For early local development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let graphql_routes = web::graphql_routes::routes();

    let app = Router::new()
        .route("/graphiql", get(graphiql))
        .nest("/graphql", graphql_routes);

    // region:    --- Start Server

    debug!("{:<12} - 0.0.0.0:3000\n", "LISTENING");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // endregion: --- Start Server

    Ok(())
}
