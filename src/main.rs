use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use std::fmt::{self, Display};

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap(); //creating a TcpListener and biding it to a port
                                                                 // and crashing the program if the binding throws an error
                                                                 // otherwise assining the TcpListener to the listener variable
    
    let json_file = fs::read_to_string("accounts.json").unwrap(); //reading data from file
    let mut accounts:Vec<Account> = Vec::new(); 
 
    for line in json_file.lines(){
        match Account::new(line){
            Some(acc)=>accounts.push(acc),
            None=>()
        };
    }


    for stream in listener.incoming(){ //incoming method returns an iterator of TcpStreams
        let stream = stream.unwrap();  //a single TcpStream is an open connection between the client and the server
                            //^ we terminate the program if it catches an error
            
            handle_connection(stream, &mut accounts);
          
        }
}

fn handle_connection(mut stream: TcpStream, accounts: &mut Vec<Account>){
    let mut buffer:Vec::<u8> = vec![0; 2048]; //suitable buffer for now
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
        let request_splitup = request.split_whitespace().collect::<Vec<_>>();


        match request_splitup[1][1..].find('/'){
            None=>{
            //creating a response from the contents of the file, and the status line
            content = String::from("{\r\n\t\"accounts\": [");
            for i in accounts{
                content = format!("{},\r\n\t\t{}", content, i);
            }
            content = content.replacen(',', "", 1) +"\r\n\t]\r\n}";
            status_line = "HTTP/1.1 200 OK\r\n".to_string();
            }

            Some(id)=>{
                match request_splitup[1][(id+2)..].find('/')
                {
                    Some(end_of_id)=>{//handling POST requests and updates
                       
                        let person_id =  request_splitup[1][(id+2)..][..(end_of_id)].to_string();
                        
                        if request_splitup[1][(id+2)..][(end_of_id+1)..] == *"update"{
                            let start_of_data= String::from_utf8_lossy(&buffer[..]).find('\'').unwrap();                            
                            let post_input_data = String::from_utf8_lossy(&buffer[..])[(start_of_data+1)..].to_string();

                            match person_id.parse::<usize>(){
                                Err(_)=>{status_line="HTTP/1.1 404 Not Found\r\n".to_string();},
                                Ok(s)=>{
                                    
                                    for acc in accounts {
                                        if acc.id == s{
                                            
                                            if acc.update( &post_input_data[..]){
                                                status_line = "HTTP/1.1 200 OK\r\n".to_string();
                                                content="".to_string();
                                                
                                            }else{
                                                status_line= "HTTP/1.1 400 Bad Request\r\n".to_string();
                                                content="*****The json will not work, if there are any spaces between the curly brackets, because the reading will block, and the response will be \"400 Bad Request\"******".to_string();
                                            }
                                        }
                                    }
                                }
                            }


                        }else{
                            status_line="HTTP/1.1 404 Not Found\r\n".to_string();
                            content="".to_string();
                        }


                    },
                    None=>{
                        let person_id = request_splitup[1][(id+2)..].parse::<usize>().unwrap();
                        status_line = "HTTP/1.1 200 OK\r\n".to_string();
                        let mut was_found = false;
                        for id_of_accounts in accounts{
                            if person_id == id_of_accounts.id{
                                content = format!("{}", id_of_accounts);
                                was_found = true;
                                break;
                            } 
                        }
                        if !was_found{
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


struct Account{
    id:usize,
    firstname: String,
    lastname:String,
}
impl Account{
    fn new(line: &str)->Option<Account>{
        let mut id:usize = 0;
        let mut firstname:String = "".to_string();
        let mut lastname:String = "".to_string();
        
        let line_setup = line.replace('{', "")
                              .replace('}', "")
                              .replace(' ', "")
                              .replace('[', "")
                              .replace(']', "");
        if line_setup.is_empty(){
            return None;
        }
        let line_setup = line_setup.split(',').collect::<Vec<_>>();
        if !line_setup[0].contains(':')
        {
            return None; 
        }
        for data_pair in line_setup{ 
            let identifier_value = data_pair.split(':').collect::<Vec<_>>();
            if identifier_value[0].replace('"', "").to_string()=="id".to_string(){
                id=identifier_value[1].replace('"', "").parse::<usize>().unwrap();
            }else if identifier_value[0].replace('"', "")=="firstname"{
                firstname=identifier_value[1].replace('"', "").to_string();
            }else if identifier_value[0].replace('"', "")=="lastname"{
                lastname=identifier_value[1].replace('"', "").to_string();
            }else if identifier_value[0].replace('"', "")==""{
                
            }
            else{
                return None;
            }
        }

        Some(Account{
            id, 
            firstname, 
            lastname
        })
    }

    fn update(&mut self, line:&str)->bool{
        let created_from_line =Account::new(line);
        match created_from_line{
            None=>{return false;},
            Some(acc)=>{
                
                if acc.firstname != "" {    
                    self.firstname = acc.firstname;
                }
                if acc.lastname != ""{
                    self.lastname =acc.lastname;
                }
                true
            }
        }
    }
    
}

impl Display for Account{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(f, "{{ \"id\": \"{}\", \"firstname\": \"{}\", \"lastname\": \"{}\" }}", self.id, self.firstname, self.lastname)
    }
}

