use axum::Router;
use axum::response::Html;
use axum::routing::get;
use std::env;

/// Ideally, `#[tokio::main]` will work regardless of target by
/// determining that for Wasi, the flavor must be
/// `tokio::runtime::current_thread`.
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::net::SocketAddr;

        tracing_subscriber::fmt::init();
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app().into_make_service())
            .await?;
    }

    #[cfg(target_arch = "wasm32")]
    {
        use std::os::wasi::io::FromRawFd;

        tracing_subscriber::fmt::init();
        let std_listener = unsafe { std::net::TcpListener::from_raw_fd(3) };
        std_listener.set_nonblocking(true).unwrap();
        axum::Server::from_tcp(std_listener)
            .unwrap()
            .serve(app().into_make_service()).await?;
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use std::os::wasi::io::FromRawFd;

    tracing_subscriber::fmt::init();
    let std_listener = unsafe { std::net::TcpListener::from_raw_fd(3) };
    std_listener.set_nonblocking(true).unwrap();
    axum::Server::from_tcp(std_listener)
        .unwrap()
        .serve(app().into_make_service()).await?;

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