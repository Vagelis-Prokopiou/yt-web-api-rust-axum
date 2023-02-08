use axum::routing::get;
use axum::Router;
use tokio::runtime::Builder;

use api_axum::get_users_route;

async fn get_root_route() -> &'static str { return "Hello from root"; }
async fn get_any_route() -> &'static str { return "Hello from any route"; }

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
            .route("/", get(get_root_route));

        println!("\nServer running at: http://0.0.0.0:3000/users");
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}
