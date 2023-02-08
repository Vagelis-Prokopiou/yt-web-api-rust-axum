use axum::body::{Body, Bytes};
use axum::http::{Request, StatusCode};
use axum::response::Response;
use axum::routing::{get, post};
use axum::{body, Router};
use tokio::runtime::Builder;

use api_axum::get_users_route;

async fn get_root_route(req: Request<Body>) -> &'static str {
    println!("req: {:?}", req);
    return "Hello from root";
}
async fn get_any_route() -> &'static str { return "Hello from any route"; }

// Rquest: curl -d "param1=value1&param2=value2" -X POST http://localhost:3000/data
async fn post_any_route(mut req: Request<Body>) -> String {
    println!("req: {:?}", req);
    let body = req.body_mut();
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    let body_str = std::str::from_utf8(&*bytes).unwrap();
    return format!("\nHello from post any route with body: {}\n", body_str);
}
fn main() {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(num_cpus::get_physical())
        .build()
        .unwrap();

    runtime.block_on(async {
        let app = Router::new()
            .route("/users", get(get_users_route))
            .route("/*path", get(get_any_route))
            .route("/*path", post(post_any_route))
            .route("/", get(get_root_route));

        println!("\nServer running at: http://0.0.0.0:3000/users");
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}
