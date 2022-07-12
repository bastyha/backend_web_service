use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

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
    stream.read(&mut buffer).unwrap(); //get the data (as bytes) from the TcpStream and putting them in a buffer
    
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    
    //converting the response to bytes and sending it
    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();//prevents the program from continuing whil the bytes from the response are being sent 
}