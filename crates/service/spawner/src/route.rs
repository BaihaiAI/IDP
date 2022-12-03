use axum::routing::post;
use axum::Router;

pub fn init_router() -> Router {
    Router::new().route(
        "/start_kernel",
        post(crate::handler::start_kernel::start_kernel),
    )
}
