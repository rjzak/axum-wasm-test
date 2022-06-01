use axum::Router;
use axum::response::Html;
use axum::routing::get;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::net::{IpAddr, SocketAddr};
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app().into_make_service())
            .await?;
    }

    #[cfg(target_arch = "wasm32")]
    {
        use std::os::wasi::io::FromRawFd;
        let std_listener = unsafe { std::net::TcpListener::from_raw_fd(3) };
        std_listener.set_nonblocking(true).unwrap();
        axum::Server::from_tcp(std_listener)
            .unwrap()
            .serve(app().into_make_service()).await?;
    }

    Ok(())
}

fn app() -> Router {
    Router::new()
        .route("/", get(|| async {
            Html(format!("Hello, world from {}!\n<br/>Time: {:?}", platform(), chrono::offset::Local::now()).to_string())
        }))
}

fn platform() -> String {
    let mut name = env::consts::ARCH.to_string();
    if env::consts::OS.len() > 0 {
        name = format!("{}-{}", name, env::consts::OS);
    }
    name
}