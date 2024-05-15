// region:    --- Modules

use async_graphql::{
    ComplexObject, Context, EmptyMutation, EmptySubscription, Guard, Object, Schema, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, routing::{post}, Router};

// endregion: --- Modules

struct Query;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Role {
    Admin,
    Guest,
}

struct RoleGuard {
    roles: Vec<Role>,
}

impl RoleGuard {
    fn new(roles: Vec<Role>) -> Self {
        Self { roles }
    }
}

impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        Ok(())
    }
}

#[Object]
impl Query {
    #[graphql(guard = "RoleGuard::new(vec![Role::Admin])")]
    async fn books(&self) -> Vec<Book> {
        books(None).await
    }

    async fn authors(&self) -> Vec<Author> {
        authors().await
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct Book {
    id: String,
    title: String,
    author_id: String,
}

#[ComplexObject]
impl Book {
    async fn author(&self) -> Author {
        authors().await[0].clone()
    }
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
struct Author {
    id: String,
    name: String,
    book_ids: Vec<String>,
}

#[ComplexObject]
impl Author {
    async fn books(&self) -> Vec<Book> {
        books(None).await
    }
}

async fn books(author_id: Option<String>) -> Vec<Book> {
    let books = vec![Book {
        id: "1".to_string(),
        title: "The Hobbit".to_string(),
        author_id: "1".to_string(),
    }];
    if let Some(author_id) = author_id {
        return books
            .into_iter()
            .filter(|book| book.author_id == author_id)
            .collect();
    }
    books
}

async fn authors() -> Vec<Author> {
    vec![Author {
        id: "1".to_string(),
        name: "J.R.R. Tolkien".to_string(),
        book_ids: vec!["1".to_string()],
    }]
}

pub fn routes() -> Router {
    Router::new()
        .route("/", post(graphql_handler))
        .with_state(Schema::new(Query, EmptyMutation, EmptySubscription))
}

async fn graphql_handler(
    State(schema): State<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
