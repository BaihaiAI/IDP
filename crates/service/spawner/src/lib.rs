mod handler;
mod route;

pub async fn main() {
    let app = route::init_router();

    let address =
        std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, business::spawner_port()));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
