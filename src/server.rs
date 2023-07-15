use hyper::{Body, Request, Response, Server};
use tokio::net::TcpListener;

static BLOCKED_WEBSITE: &str = "https://google.com/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    start_server("127.0.0.1:8080").await
}

async fn start_server(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a TCP listener which will listen for incoming connections
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

async fn handle_client(socket: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let http = hyper::server::conn::Http::new();
    let conn = http.serve_connection(
        socket,
        hyper::service::service_fn(|req| async { handle_request(req).await }),
    );

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("server connection error: {}", e);
        }
    });

    Ok(())
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let host = req.uri().host().unwrap_or("");

    if host == "andrescn.me" || host.ends_with(BLOCKED_WEBSITE) {
        let response = Response::builder()
            .status(403)
            .body(Body::from("Blocked"))
            .unwrap();

        Ok(response)
    } else {
        let client = hyper::Client::new();
        client.request(req).await.map(|e| {
            println!("e: {:?}", e);
            e
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use std::net::SocketAddr;
    use tokio::task::JoinHandle;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_blocking_sites() {
        // Start the proxy server in a separate task
        let server_handle = tokio::spawn(async {
            start_server("127.0.0.1:8080").await.unwrap();
        });

        // Give the proxy server a bit of time to start up
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Create a HTTP client
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .proxy(reqwest::Proxy::http("http://localhost:8080").unwrap())
            .build()
            .unwrap();

        // Test blocked site
        let blocked_res = client.get(BLOCKED_WEBSITE).send().await.unwrap();
        assert_eq!(blocked_res.status().as_u16(), 403);

        // Test non-blocked site
        let allowed_res = client.get("http://google.com").send().await.unwrap();
        assert_eq!(allowed_res.status().as_u16(), 200);

        // Stop the proxy server
        server_handle.abort();
    }
}
