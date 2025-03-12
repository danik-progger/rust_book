use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use _21_web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    }else if buf.starts_with(sleep) {
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let port: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind(port).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
