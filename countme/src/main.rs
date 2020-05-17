//use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::{str, thread};
use std::env;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicU32;
use crossbeam_queue::ArrayQueue;
use std::sync::Arc;
use std::time::Duration;
use threadpool::ThreadPool;

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
                if size == 0 {
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
    stream.shutdown(Shutdown::Both).unwrap();

    match post_request {
        Some(r) => {
            let vec: Vec<&str> = r.split("\n").collect();
            let body: u32 = vec.last().unwrap().parse().unwrap();
            SUM.fetch_add(counter * body, Ordering::SeqCst);
        }
        _ => ()
    }
}

fn main() {
    let port = env::var("PORT").unwrap_or(String::from("80"));
    let listener = Arc::new(TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap());

    let q = Arc::new(ArrayQueue::<TcpStream>::new(1000));

    let cloned_listener = Arc::clone(&listener);
    let cloned_q = Arc::clone(&q);

    thread::spawn(move || {
        for stream in cloned_listener.incoming() {
            if let Ok(stream) = stream {
                if let Err(e) = cloned_q.push(stream) {
                    println!("{:?}", e);
                }
            }
        }
    });

    let cloned_q = Arc::clone(&q);
    let pool = ThreadPool::new(150);
    loop {
        if let Ok(stream) = cloned_q.pop() {
            stream.set_read_timeout(Some(Duration::from_millis(40))).unwrap();
            pool.execute(move|| {
                handle_client(stream)
            });
        }
    }
}


