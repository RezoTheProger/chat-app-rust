#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, rc::Rc};
use slint::{ModelRc, SharedString, VecModel};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio_tungstenite::accept_async;
slint::include_modules!();


const ADDR:&str = "localhost:6666";


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();


    //entering tcp stream
    if let Ok(mut stream) = TcpStream::connect(ADDR).await{
            println!("Connected to TCP stream: {}{}", stream.local_addr().unwrap().ip(),stream.local_addr().unwrap().ip());
            let gui = ui_handle.unwrap();


               let mut buffer:[u8;1024] = [0;1024];
               let _ = stream.read(&mut buffer).await.unwrap(); 
               let message:String = String::from_utf8_lossy(&buffer).to_string();
               let mut message_vector: Vec<String> = vec![];
                message_vector.push(message);
                let the_model : Rc<VecModel<SharedString>> =Rc::new(VecModel::from(message_vector.into_iter().map(|messages| messages.into()).collect::<Vec<SharedString>>()));
                let the_model_rc = ModelRc::from(the_model.clone());

                gui.set_messages(the_model_rc);
            //if yk how to read its called on_send_message for a reason
            gui.on_send_message({

                async move |message: SharedString| {
                let msg_clone = message.clone();
                let _ = stream.write(msg_clone.to_string().as_bytes()).await;
                let _ = stream.flush().await;
                println!("sent message: {}", msg_clone);
        }
    });

    ui.run()?;
    }
    else{
        println!("Unsuccessful connection to TCP stream: {}",ADDR);
    }

    // Run the UI loop on the main thread



    Ok(())



    }
