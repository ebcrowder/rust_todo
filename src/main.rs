use self::models::{NewTodo, Todo};
use bytes::buf::BufExt;
use diesel::prelude::*;
use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use std::env;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

async fn api_post_response(req: Request<Body>, conn: PgConnection) -> Result<Response<Body>> {
    use schema::todos;
    // Aggregate the body...
    let whole_body = hyper::body::to_bytes(req.into_body()).await?;
    // Decode as JSON...
    let json: NewTodo = serde_json::from_reader(whole_body.reader())?;

    let new_todo = NewTodo { title: json.title };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result::<(i32, String, bool)>(&conn)
        .expect("Error saving new post");

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        // TODO fix this
        .body(Body::from("todo inserted successfully."))?;
    Ok(response)
}

async fn api_get_response(conn: PgConnection) -> Result<Response<Body>> {
    use schema::todos::dsl::*;

    let results = todos.load::<Todo>(&conn).expect("error loading todos");

    let mut todo_vec: Vec<Todo> = Vec::new();

    for todo in results {
        todo_vec.push(Todo {
            id: todo.id,
            title: todo.title,
            done: todo.done,
        })
    }

    let res = match serde_json::to_string(&todo_vec) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    };
    Ok(res)
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    let conn = establish_connection();
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/todos") => api_post_response(req, conn).await,
        (&Method::GET, "/todos") => api_get_response(conn).await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let addr = "127.0.0.1:8000".parse().unwrap();

    let new_service = make_service_fn(move |_| {
        // Move a clone of `client` into the `service_fn`.
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                response_examples(req)
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
