// zproxy.rs
// use: zingo-proxy - recieves Grpc calls, passes Grpc calls to lightwalletd/zebrad, returns Grpc response.
//

use http::Uri;
use std::thread;
use std::time::Duration;
use zingo_proxy::zproxy_utils::{spawn_server, ProxyServer};

#[tokio::main]
async fn main() {
    let server_port = 8080;
    let server_handle = spawn_server(server_port, 9067, 18232).await;
    loop {
        thread::sleep(Duration::from_secs(10));
    }
}
