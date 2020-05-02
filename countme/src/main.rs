//! A "hello world" echo server with Tokio
//!
//! This server will create a TCP listener, accept connections in a loop, and
//! write back everything that's read off of each TCP connection.
//!
//! Because the Tokio runtime uses a thread pool, each TCP connection is
//! processed concurrently with all other TCP connections across multiple
//! threads.
//!
//! To see this server in action, you can run this in one terminal:
//!
//!     cargo run --example echo
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect 127.0.0.1:8080
//!
//! Each line you type in to the `connect` terminal should be echo'd back to
//! you! If you open up multiple terminals running the `connect` example you
//! should be able to see them all make progress simultaneously.

#![warn(rust_2018_idioms)]

use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;
use regex::Regex;

static mut SUM: i32 = 0;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:80".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let mut listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    let lower_g = 71;
    let upper_g = 103;
    let post_response = "HTTP/1.1 200 OK\n\n".as_bytes();

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let mut buf = [0; 150];
            let re = Regex::new(r"(\d+)").unwrap();

            // In a loop, read data from the socket and write the data back.
            loop {
                let length = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if length == 0 {
                    return;
                }
                unsafe {
                    if buf[0] == lower_g || buf[0] == upper_g {
                        socket
                            .write_all(format!("HTTP/1.1 200 OK\n\n{value}\r\n", value = SUM).as_bytes())
                            .await
                            .expect("failed to write data to socket");
                        break;


                    } else {
                        let q = String::from_utf8(buf[length - 10..length].to_vec()).unwrap();
                        let foo = &q;
                        let w = re.find(foo);
                        SUM += w.unwrap().as_str().parse::<i32>().unwrap();
                        //println!("{}", SUM);
                        socket
                            .write_all(post_response)
                            .await
                            .expect("failed to write data to socket");
                        break;
                    }
                }
            }
        });
    }
}