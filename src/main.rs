use axum::{
    body::{boxed, Body},
    http::{Response, StatusCode},
    routing::{get, post, put},
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

mod auth;
mod db;
mod entities;
mod errors;
mod models;
mod openapi;
mod repositories;
mod routes;
mod services;
mod utils;

lazy_static::lazy_static! {
    pub static ref JWT_SECRET: String =
        env::var("JWT_SECRET").expect("Missing env var: JWT_SECRET");
}

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
    let _ = dotenv::dotenv().ok();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    tracing_subscriber::fmt::init();

    let db_conn = db::get_db_conn(opt.db_conn).await;

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/auth/token", post(routes::get_token))
                .route("/users", post(routes::create_user))
                .route("/users/me", get(routes::get_logged_user))
                .route("/users/me", put(routes::update_logged_user)),
        )
        .merge(openapi::swagger_ui())
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
