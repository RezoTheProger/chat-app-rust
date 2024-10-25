use std::borrow::Cow;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

const SERV_ADDR:&str = "127.0.0.1:6666";

#[tokio::main]
async fn main() {
    println!("Starting : {} ", SERV_ADDR);
    
    let listener = TcpListener::bind(SERV_ADDR).await.unwrap();
    println!("binded successfully with: {}{}", listener.local_addr().unwrap().ip(), listener.local_addr().unwrap().port());
       


    match listener.accept().await{
        Ok(( stream,socket)) => {
            println!("user from addr: {} connected successfully!",socket);
            handle_connection(stream).await;
        },
        Err(e) => println!("couldn't get a connection, error occured: {}",e),
    }

}
        
async fn handle_connection(mut stream:TcpStream){
    let mut buffer:[u8;1024] = [0;1024];
    
    let len = stream.read(&mut buffer).await.unwrap();
    let message: Cow<str>= String::from_utf8_lossy(&buffer[..len]);
    println!("got message: {}", message);


    let _ =stream.write_all(message.as_bytes()).await;
    println!("sent message: {} successfully!",message);

}
