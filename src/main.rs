use axum::{
    body::{boxed, Body},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use clap::Parser;
use log::info;
use std::{
    env,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{services::ServeDir, trace::TraceLayer};

mod db;
mod models;
mod repositories;
mod routes;
mod services;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "fichawork server")]
struct Opt {
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,
    #[clap(short = 's', long = "static-dir", default_value = "./frontend/dist")]
    static_dir: String,
    #[clap(
        short = 'd',
        long = "db-conn",
        default_value = "postgres://postgres@localhost/fichawork"
    )]
    db_conn: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    tracing_subscriber::fmt::init();

    let db_conn = db::get_db_conn(opt.db_conn).await;

    let app = Router::new()
        .route("/test/hello", get(hello))
        .nest("/api", Router::new().route("/dbtest", get(routes::dbtest)))
        .with_state(db_conn)
        .fallback_service(get(|req| async move {
            match ServeDir::new(opt.static_dir).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("Error: {}", err))))
                    .expect("error response"),
            }
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
    "hello"
}
