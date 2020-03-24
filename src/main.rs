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
    let data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    // TODO look at serde_json impl
    let json = serde_json::to_string(&data)?;

    let new_todo = NewTodo { title: json.title };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(&conn)
        .expect("Error saving new post");

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;
    Ok(response)
}

async fn api_get_response() -> Result<Response<Body>> {
    let data = vec!["foo", "bar"];
    let res = match serde_json::to_string(&data) {
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

async fn response_examples(req: Request<Body>, conn: PgConnection) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/json_api") => api_post_response(req, conn).await,
        (&Method::GET, "/json_api") => api_get_response().await,
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

    let conn = establish_connection();

    let addr = "127.0.0.1:8000".parse().unwrap();

    let new_service = make_service_fn(move |_| {
        // Move a clone of `client` into the `service_fn`.
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                response_examples(req, conn)
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
