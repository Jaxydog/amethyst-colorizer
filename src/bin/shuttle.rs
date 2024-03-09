use axum::Router;
use shuttle_axum::ShuttleAxum;

#[shuttle_runtime::main]
async fn shuttle_main() -> ShuttleAxum {
    Ok(Router::new().into())
}
