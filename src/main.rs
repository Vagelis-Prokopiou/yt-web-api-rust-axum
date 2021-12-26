use axum::routing::get;
use axum::Router;
use tokio::runtime::Builder;

use api_axum::get_users;

fn main() {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(num_cpus::get_physical())
        .build()
        .unwrap();

    runtime.block_on(async {
        let app = Router::new().route("/users", get(get_users));

        println!("\nServer running at: http://0.0.0.0:3000/users");
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}
