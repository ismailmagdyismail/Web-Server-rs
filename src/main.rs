use std::{
    error::Error,
    io::{Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let address = "0.0.0.0:8080";
    let listener = std::net::TcpListener::bind(&address)?;

    println!("server is listening on adress {}", address);

    for stream_result in listener.incoming() {
        let mut client_stream = stream_result?;
        println!("connection accepted, new client created !!");

        let mut buffer = [b'0'; 1024];
        let read_bytes = client_stream.read(&mut buffer)?;
        println!(
            "total read bytes from client {} with content {}",
            read_bytes,
            std::str::from_utf8(&buffer).expect("invalid http request, not valid utf8!!!"),
        );

        let response = create_response();
        let written_bytes = client_stream.write(response.as_bytes())?;
        println!("response of size {}, is written back to the client", {
            written_bytes
        });

        println!("==============================");
    }
    Ok(())
}

fn create_response() -> String {
    let body = "<!DOCTYPE html><html><body>Hello World from RUST!</body></html>";
    let response_header = format!(
        "HTTP/1.1 200 OK
Date: Mon, 21 Sep 2026 23:26:00 GMT
Server: Apache/2.4.41 (Ubuntu)
Content-Type: text/html; charset=UTF-8
Content-Length: {}
Connection: keep-alive",
        body.len()
    );

    let response = (response_header + "\r\n\r\n") + body;
    response
}
