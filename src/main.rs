use axum::routing::{get};
use axum::{Json, Router};
use tokio::runtime::Builder;

#[derive(serde::Serialize)]
pub struct User {
    pub(crate) id: u16,
    pub(crate) age: u16,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) framework: String,
}

pub async fn get_users_route() -> Json<Vec<User>> {
    let mut users = Vec::with_capacity(1000);
    for index in 1..1001_u16 {
        users.push(User {
            id: index,
            age: 25,
            first_name: format!("first_name{}", index),
            last_name: format!("last_name{}", index),
            framework: "Axum".to_owned(),
        })
    }
    Json(users)
}
fn main() {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(num_cpus::get_physical())
        .build()
        .unwrap();

    runtime.block_on(async {
        let app = Router::new()
            .route("/", get(|| async { "Hello from root" }))
            .route("/api/users", get(get_users_route));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
        println!("listening on http://{}/api/users", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
    });
}
