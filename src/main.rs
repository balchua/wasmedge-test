use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{Read, Write},
};

use rand::prelude::*;

use std::net::SocketAddr;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

use urlencoding::decode;

pub fn get_random_i32() -> i32 {
    let x: i32 = random();
    return x;
}

pub fn get_random_bytes() -> Vec<u8> {
    let mut rng = thread_rng();
    let mut arr = [0u8; 128];
    rng.fill(&mut arr[..]);
    return arr.to_vec();
}

pub fn print_env() {
    println!("The env vars are as follows.");
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    println!("The args are as follows.");
    for argument in env::args() {
        println!("{}", argument);
    }
}

pub fn create_file(path: &str, content: &str) {
    let mut output = File::create(path).unwrap();
    output.write_all(content.as_bytes()).unwrap();
}

pub fn read_file(path: &str) -> String {
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => s,
        Err(e) => e.to_string(),
    }
}

pub fn del_file(path: &str) {
    fs::remove_file(path).expect("Unable to delete");
}

async fn do_echo(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("echo")))
}

async fn reverse_echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await?;

    let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
    Ok(Response::new(Body::from(reversed_body)))
}

async fn hello(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let q = req.uri().query().unwrap();
    let kv: HashMap<&str, &str> = q
        .split_whitespace()
        .map(|s| s.split_at(s.find("=").unwrap()))
        .map(|(key, val)| (key, &val[1..]))
        .collect();
    let user = kv.get("user").unwrap();
    let hello = format!("Hello {}", decode(&user).expect("UTF-8"));
    Ok(Response::new(Body::from(hello)))
}

async fn service(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/echo") => do_echo(req).await,
        (&Method::POST, "/echo/reversed") => reverse_echo(req).await,
        (&Method::POST, "/hello") => hello(req).await,
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");
    print_env();

    let x = get_random_i32();
    println!("The random i32 is {}", x);

    let bytes = get_random_bytes();
    println!("The random bytes are {:?}", bytes);

    create_file("tmp.txt", "This is in a file");
    println!("File content is ==> {}", read_file("tmp.txt"));
    //del_file("tmp.txt");

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(service))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
