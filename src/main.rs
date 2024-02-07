use std::{io::Read, net::{TcpListener, TcpStream}};
use std::io::prelude::*;
use std::fs;

fn main() {
    //? iniciar el servidor
    let addres = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addres).unwrap();
    println!("Server listening on port: {}", &addres);

    //? escuchar por conexiones
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        handle_conection(stream);
    }
}

//? manejar estas conexiones
fn handle_conection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Connection established!");
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1"; //127.0.0.1:3000/index
    if buffer.starts_with(get) {
        send_index(stream);
    } else {
        send_not_found(stream);
    }

}

fn send_index(mut stream: TcpStream) {

    let contents = fs::read_to_string("index.html").unwrap();
    stream.write(build_response(contents).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn send_not_found(mut stream: TcpStream) {
    let contents = fs::read_to_string("404.html").unwrap();
    stream.write(build_response(contents).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn build_response(content: String) -> String {
     format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    )
}
