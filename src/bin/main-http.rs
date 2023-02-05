use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let _cert = std::fs::read_to_string("cert.pem");
    println!("Loaded cert, starting web server");
    tokio::select! {
        _ = start_normal_server(8080) => {
            println!("the web server shut down")
        }
        _ = start_control_server(3000) => {
            println!("the control server shut down")
        }
    };
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

async fn start_control_server(port: u32) {
    // build our application
    let app = Router::new().route(
        "/reload_certs",
        post(|| async {
            println!("Reloading cert");
            match std::fs::read_to_string("cert.pem") {
                Ok(_) => "Successfully reloaded cert".into_response(),
                Err(e) => {
                    let error = format!("could not reload cert: {e}");
                    eprintln!("{error}");
                    let resp = (StatusCode::INTERNAL_SERVER_ERROR, error);
                    resp.into_response()
                }
            }
        }),
    );

    // run it
    let addr = format!("0.0.0.0:{port}").parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
