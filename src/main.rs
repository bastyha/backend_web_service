use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap(); //creating a TcpListener and biding it to a port
                                                                 // and crashing the program if the binding throws an error
                                                                 // otherwise assining the TcpListener to the listener variable

    for stream in listener.incoming(){ //incoming method returns an iterator of TcpStreams
        let stream = stream.unwrap();  //a single TcpStream is an open connection between the client and the server
                            //^ we terminate the program if it catches an error
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024]; //suitable buffer for now
    //reading request from the TCP stream
    stream.read(&mut buffer).unwrap(); //get the request (as bytes) from the TcpStream and putting them in a buffer
    
    let get = b"GET /healthz HTTP/1.1\r\n"; //checking if request is for healthz
    let accounts = b"GET /accounts HTTP/1.1\r\n";

    let mut response = String::new();
    let status_line = "HTTP/1.1 200 OK\r\n";
    if buffer.starts_with(get){ //checking what address
        response = String::from(status_line); // when /healthz sending only the status line back
    }else if buffer.starts_with(accounts){
        let content = fs::read_to_string("accounts.json").unwrap(); //reading data from file
        //creating a response from the contents of the file, and the status line
        response = format!("{}Content-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content);
    }
    stream.write(response.as_bytes()).unwrap();//sending response
    
    stream.flush().unwrap();//prevents the program from continuing while the bytes from the response are being sent 
}