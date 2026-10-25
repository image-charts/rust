use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Once;
use std::thread;

use image_charts::ImageCharts;

static PROXY_ENV: Once = Once::new();

fn unreachable_socks_proxy() {
    PROXY_ENV.call_once(|| {
        std::env::set_var("ALL_PROXY", "socks5://127.0.0.1:1");
        std::env::set_var("HTTP_PROXY", "socks5://127.0.0.1:1");
        std::env::set_var("HTTPS_PROXY", "socks5://127.0.0.1:1");
    });
}

fn serve_once(response: &'static [u8]) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut request = [0u8; 2048];
            let _ = stream.read(&mut request);
            let _ = stream.write_all(response);
        }
    });
    port
}

fn closed_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

fn client(port: u16) -> ImageCharts {
    ImageCharts::builder()
        .protocol("http")
        .host("127.0.0.1")
        .port(port)
        .build()
        .cht("p")
        .chd("t:60,40")
        .chs("100x100")
}

const OK: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\nPNG!";
const TRUNCATED: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\nPNG!";

#[cfg(feature = "async")]
mod async_api {
    use super::*;

    #[tokio::test]
    async fn socks_proxy_env_is_ignored() {
        unreachable_socks_proxy();
        let bytes = client(serve_once(OK)).to_buffer().await.unwrap();
        assert_eq!(bytes, b"PNG!");
    }

    #[tokio::test]
    async fn connection_error_keeps_its_cause() {
        let err = client(closed_port()).to_buffer().await.unwrap_err();
        assert!(err.message.to_lowercase().contains("refused"), "{}", err.message);
        assert!(!err.message.contains("http://"), "{}", err.message);
    }

    #[tokio::test]
    async fn body_error_does_not_leak_the_url() {
        let err = client(serve_once(TRUNCATED)).to_buffer().await.unwrap_err();
        assert!(!err.message.contains("127.0.0.1"), "{}", err.message);
    }
}

#[cfg(feature = "blocking")]
mod blocking_api {
    use super::*;

    #[test]
    fn socks_proxy_env_is_ignored() {
        unreachable_socks_proxy();
        let bytes = client(serve_once(OK)).to_buffer_blocking().unwrap();
        assert_eq!(bytes, b"PNG!");
    }

    #[test]
    fn connection_error_keeps_its_cause() {
        let err = client(closed_port()).to_buffer_blocking().unwrap_err();
        assert!(err.message.to_lowercase().contains("refused"), "{}", err.message);
        assert!(!err.message.contains("http://"), "{}", err.message);
    }

    #[test]
    fn body_error_does_not_leak_the_url() {
        let err = client(serve_once(TRUNCATED)).to_buffer_blocking().unwrap_err();
        assert!(!err.message.contains("127.0.0.1"), "{}", err.message);
    }
}
