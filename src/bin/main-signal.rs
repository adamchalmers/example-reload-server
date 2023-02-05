use axum::{routing::get, Router};
use std::process;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() {
    let _cert = std::fs::read_to_string("cert.pem");
    println!("Loaded cert, starting web server");
    println!("My pid is {}", process::id());
    tokio::select! {
        _ = start_normal_server(8080) => {
            println!("the web server shut down")
        }
        _ = listen_for_reload(SignalKind::hangup()) => {
            println!("the signal listener stopped")
        }
    }
}

async fn start_normal_server(port: u32) {
    // build our application
    let app = Router::new().route("/hello", get(|| async { "Hello, world!" }));

    // run it
    let addr = format!("127.0.0.1:{port}").parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn listen_for_reload(signal_kind: SignalKind) -> Result<(), std::io::Error> {
    // An infinite stream of signals.
    let mut stream = signal(signal_kind)?;

    loop {
        stream.recv().await;

        match std::fs::read_to_string("cert.pem") {
            Ok(_) => eprintln!("Successfully reloaded cert"),
            Err(e) => eprintln!("could not reload cert: {e}"),
        }
    }
}
