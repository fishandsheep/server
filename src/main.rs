use local_ip_address::local_ip;
use std::env;
use tiny_http::{Response, Server};

fn main() {
    let args: Vec<String> = env::args().collect();
    let binding = "8080".to_string();
    let port = args.get(1).unwrap_or(&binding);
    let addr = format!("0.0.0.0:{}", port);

    let server = Server::http(&addr).expect("Failed to start server");
    println!("Listening on http://{}", addr);

    for request in server.incoming_requests() {
        // 处理 remote_addr 为空的情况
        let remote_addr = request
            .remote_addr()
            .map(|addr| addr.to_string()) // 转换为 String
            .unwrap_or_else(|| "Unknown".to_string()); // 处理 None 情况
        println!("Received request from: {}", remote_addr);
        println!("Request: {:?}", request);

        // 获取本机 IP（修正）
        let local_ip = local_ip().unwrap_or_else(|_| "Unknown".parse().unwrap());
        // 生成响应
        let response_text = format!("{} response\n", local_ip);
        let response = Response::from_string(response_text);
        request.respond(response).unwrap();
    }
}
