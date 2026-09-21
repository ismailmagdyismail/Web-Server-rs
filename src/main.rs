use std::{error::Error, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let address = "0.0.0.0:8080";
    let stream = std::net::TcpListener::bind(&address)?;

    println!("server is listening on adress {}", address);


    let (mut client_stream, client_adresss) = stream.accept()?;

    println!(
        "connection accepted with address {} and port {}",
        client_adresss.ip(),
        client_adresss.port()
    );

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

    println!("response is {}", response);

    let written_bytes = client_stream.write(response.as_bytes())?;
    println!("response of size {}, is written back to the client", {
        written_bytes
    });

    Ok(())
}
