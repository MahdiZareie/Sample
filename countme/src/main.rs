//use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
use std::env;


fn handle_client(mut stream: TcpStream, so_far: u32) -> u32 {
    // this function is running to handle only one GET request or bunch of pipelined POST requests

    let mut post_request: Option<String> = None; // All post request are the same, keep first one
    let mut counter = 0u32; // only keep the count of request
    let mut data = [0 as u8; 256]; // big enough buffer to read the whole request in one go

    // pre-calculated responses
    let post_response = "HTTP/1.1 200 OK\ncontent-length: 0\n\n".as_bytes();
    let get_response_text = format!("HTTP/1.1 200 OK\n\n{}\r\n", so_far);
    let get_response = get_response_text.as_bytes();

    while match stream.peer_addr() {
        Ok(_) => true,
        _ => false
    } {
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0{
                    break;
                }
                let incoming = String::from(str::from_utf8(&data[0..size]).unwrap());
                let is_get = incoming.starts_with("G");
                if is_get {
                    stream.write(get_response).unwrap();
                    break;
                } else {
                    if post_request == None {
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
            return counter * body;
        },
        _ => {
            return 0
        }
    }

}

fn main() {
    let port = env::var("PORT").unwrap_or(String::from("80"));
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    let mut so_far = 0u32;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream.set_nonblocking(false).unwrap();
                let new_counter = handle_client(stream, so_far);
                so_far += new_counter;
            }
            Err(e) => {
                 println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}


