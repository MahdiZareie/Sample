//use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::{str, thread};
use std::env;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicU32;
use threadpool::ThreadPool;
use std::time::Duration;
use std::error::Error;

static SUM: AtomicU32 = AtomicU32::new(0);

fn handle_client(mut stream: TcpStream) -> () {
    // this function is running to handle only one GET request or bunch of pipelined POST requests

    let mut post_request: Option<String> = None; // All post request are the same, keep first one
    let mut counter = 0u32; // only keep the count of request
    let mut data = [0 as u8; 256]; // big enough buffer to read the whole request in one go

    // pre-calculated responses
    let post_response = "HTTP/1.1 200 OK\ncontent-length: 0\n\n".as_bytes();
    let get_response_text = format!("HTTP/1.1 200 OK\n\n{}\r\n", SUM.load(Ordering::SeqCst));
    let get_response = get_response_text.as_bytes();
    while stream.peer_addr().is_ok() {

        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0{
                    break;
                }
                if data[0] == b'G' {
                    stream.write(get_response).unwrap();
                    break;
                } else {
                    if post_request == None {
                        let incoming = String::from(str::from_utf8(&data[0..size]).unwrap());
                        post_request = Some(incoming);
                    }
                    counter += 1;
                    stream.write(post_response).unwrap();
                }
            }
            Err(_) => {
                break;
            }
        }
    }
    if let Err(e) = stream.shutdown(Shutdown::Both){
        println!("shutdown error: {:?}", e);
    };

    match post_request {
        Some(r) => {
            let vec: Vec<&str> = r.split("\n").collect();
            let body: u32 = vec.last().unwrap().parse().unwrap();
            SUM.fetch_add(counter * body, Ordering::SeqCst);
        },
        _ => ()
    }

}

fn main() {
    let port = env::var("PORT").unwrap_or(String::from("80"));
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    let pool = ThreadPool::new(100);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::from_millis(50))).unwrap();
                stream.set_nonblocking(false).unwrap();
                pool.execute(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
            }
        }
    }
    drop(listener);
}


