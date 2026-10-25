#![cfg(feature = "blocking")]

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

use image_charts::ImageCharts;

static ENV_LOCK: Mutex<()> = Mutex::new(());

const ENV_VARS: [&str; 9] = [
    "ALL_PROXY",
    "all_proxy",
    "HTTP_PROXY",
    "http_proxy",
    "HTTPS_PROXY",
    "https_proxy",
    "NO_PROXY",
    "no_proxy",
    "REQUEST_METHOD",
];

const DEAD_PROXY: &str = "http://127.0.0.1:1";

/// One-shot server answering 200. Returns its port and the request it received.
fn serve_once() -> (u16, JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let received = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0u8; 2048];
        let read = stream.read(&mut request).unwrap();
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\nPNG!")
            .unwrap();
        String::from_utf8_lossy(&request[..read]).to_string()
    });
    (port, received)
}

/// Runs the request to `host:port` with exactly `env` set. Returns the result.
fn fetch(env: &[(&str, String)], host: &str, port: u16) -> Result<Vec<u8>, String> {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    for name in ENV_VARS {
        std::env::remove_var(name);
    }
    for (name, proxy_url) in env {
        std::env::set_var(name, proxy_url);
    }
    ImageCharts::builder()
        .protocol("http")
        .host(host)
        .port(port)
        .build()
        .cht("p")
        .chd("t:60,40")
        .chs("100x100")
        .to_buffer_blocking()
        .map_err(|error| error.message)
}

#[test]
fn http_proxy_env_is_honoured() {
    let (proxy, received) = serve_once();
    let bytes = fetch(&[("HTTP_PROXY", format!("http://127.0.0.1:{proxy}"))], "charts.invalid", 80).unwrap();
    assert_eq!(bytes, b"PNG!");
    assert!(received.join().unwrap().contains("charts.invalid"));
}

#[test]
fn proxy_without_scheme_is_honoured() {
    let (proxy, received) = serve_once();
    let bytes = fetch(&[("HTTP_PROXY", format!("127.0.0.1:{proxy}"))], "charts.invalid", 80).unwrap();
    assert_eq!(bytes, b"PNG!");
    assert!(received.join().unwrap().contains("charts.invalid"));
}

#[test]
fn all_proxy_wins_over_http_proxy() {
    let (proxy, received) = serve_once();
    let env = [
        ("ALL_PROXY", format!("http://127.0.0.1:{proxy}")),
        ("HTTP_PROXY", DEAD_PROXY.to_string()),
    ];
    assert_eq!(fetch(&env, "charts.invalid", 80).unwrap(), b"PNG!");
    assert!(received.join().unwrap().contains("charts.invalid"));
}

#[test]
fn socks_all_proxy_falls_back_to_http_proxy() {
    let (proxy, received) = serve_once();
    let env = [
        ("ALL_PROXY", "socks5://127.0.0.1:1".to_string()),
        ("HTTP_PROXY", format!("http://127.0.0.1:{proxy}")),
    ];
    assert_eq!(fetch(&env, "charts.invalid", 80).unwrap(), b"PNG!");
    assert!(received.join().unwrap().contains("charts.invalid"));
}

#[test]
fn no_proxy_goes_direct() {
    for (name, no_proxy) in [
        ("NO_PROXY", "127.0.0.1"),
        ("no_proxy", "127.0.0.1"),
        ("NO_PROXY", "127.0.0.0/8"),
        ("NO_PROXY", "example.com,127.0.0.1"),
    ] {
        let (server, _) = serve_once();
        let env = [
            ("HTTP_PROXY", DEAD_PROXY.to_string()),
            (name, no_proxy.to_string()),
        ];
        assert_eq!(fetch(&env, "127.0.0.1", server).unwrap(), b"PNG!", "{name}={no_proxy}");
    }
}

#[test]
fn cgi_ignores_http_proxy() {
    let (server, _) = serve_once();
    let env = [
        ("REQUEST_METHOD", "GET".to_string()),
        ("HTTP_PROXY", DEAD_PROXY.to_string()),
    ];
    assert_eq!(fetch(&env, "127.0.0.1", server).unwrap(), b"PNG!");
}
