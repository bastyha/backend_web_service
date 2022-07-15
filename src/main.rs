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
    
    let mut status_line = String::new();
    let mut content = String::new();

    if buffer.starts_with(get){ //checking what address
        status_line = "HTTP/1.1 200 OK\r\n".to_string();
        content = "".to_string();
    }else {
        let request = String::from_utf8_lossy(&buffer[..]);
        let getting_account_id = request.split_whitespace().collect::<Vec<_>>();
        let json_file = fs::read_to_string("accounts.json").unwrap(); //reading data from file
        

        match getting_account_id[1][1..].find('/'){
            None=>{
            //creating a response from the contents of the file, and the status line
            content = json_file;
            status_line = "HTTP/1.1 200 OK\r\n".to_string();
            }
            Some(id)=>{

                let person_id = format!("\"id\": \"{}\"", getting_account_id[1][(id+2)..].to_string());
                
                for line in json_file.lines(){
                    match line.find(person_id.as_str()){
                        Some(_)=>{
                           status_line = "HTTP/1.1 200 OK\r\n".to_string();
                           content = line.trim_end().to_string();
                           break;
                        }
                        None=>{
                            status_line ="HTTP/1.1 404 Not Found\r\n".to_string();
                            content="".to_string();
                        }
                    }
                }
            
            }
        };
    }

    let response =             
    format!("{}Content-Length: {}\r\n\r\n{}",
    status_line,
    content.len(),
    content);

    stream.write(response.as_bytes()).unwrap();//sending response
    
    stream.flush().unwrap();//prevents the program from continuing while the bytes from the response are being sent 
}


