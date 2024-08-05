
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream){
    // This is a buffer to read data from the client
    let mut buffer = [0;1024];

    // Read the data from the client and store it in the buffer
    stream.read(&mut buffer).expect("Failed to read from client!");

    // Convert the buffer to a string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response = "Hello, client".as_bytes();
    stream.write(response).expect("Failed to write response!");
}

//Entry point
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080")
    .expect("Failed to create listener!");

    println!("Server listening on port 8080");
    for stream in listener.incoming(){
        match stream{
            Ok(stream) =>{
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
}