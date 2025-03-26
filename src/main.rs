use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper::body::to_bytes;

async fn handle_request(req: Request<Body>, addr: SocketAddr) -> Result<Response<Body>, Infallible> {
    // 先克隆 request 以便后续打印
    let req_info = format!("{:?}", req); 

    // 提取 body
    let body_bytes = to_bytes(req.into_body()).await.unwrap_or_default();
    let body_str = String::from_utf8_lossy(&body_bytes);

    println!("Received request from: {}", addr);
    println!("Request:\n{}", req_info);
    println!("Body:\n{}", body_str);

    Ok(Response::new(Body::from("Request received")))
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = ([0, 0, 0, 0], 8080).into();
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");

    let make_svc = make_service_fn(move |conn: &hyper::server::conn::AddrStream| {
        let remote_addr = conn.remote_addr();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| handle_request(req, remote_addr)))
        }
    });

    let server = Server::from_tcp(listener.into_std().unwrap())
        .unwrap()
        .serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
