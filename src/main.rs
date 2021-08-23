use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap(); // this is 1 connection
        println!("Connection established!!");
        handle_connection3(stream);
    }
}

fn handle_connection3(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let mut file = File::open("hello.html").unwrap();

    let reply = "todo: reply here with data";

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", reply);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]))
}

// ask KJ
fn handle_connection2(mut stream: TcpStream) {
    // FIXME: why doesn't it work if i don't successfully read the buffer?
    // let mut buffer = [0; 512];

    // stream.read(&mut buffer).unwrap();

    let mut file = File::open("hello.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]))
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    /**
    *  HTTP-Version Status-Code Reason-Phrase CRLF
       headers CRLF
       message-body

       CRLF === \r\n
       but because no headers, therefore CRLFCRLF --> \r\n\r\n
    */
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // flush will wait until all bytes have been written to the connection, or "flush until there's nothing left"

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]))

    // holy shit http is a text protocol
    // so if you sent a string with the right format it works!
}
