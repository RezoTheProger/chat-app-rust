#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, rc::Rc};
use slint::{ModelRc, SharedString, VecModel};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::accept_async;
slint::include_modules!();
use futures::SinkExt;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let ui = AppWindow::new()?;
    let (tx, mut rx) = mpsc::unbounded_channel();
    let ui_handle = ui.as_weak();
    
    // Sending messages from the UI to Tokio task
    ui.on_send_message({
        let tx_clone = tx.clone();
        move |message: SharedString| {
            let value = tx_clone.clone();
            tokio::spawn(async move {
                value.send(message.to_string()).expect("receiver quit");
            });
        }
    });

    // Receiving messages from the Tokio task and updating the UI
    tokio::spawn(async move {
        let mut message_vector:Vec<String> = vec![];
        while let Some(received_message) = rx.recv().await 
        {
            if let Some(ui) = ui_handle.upgrade() {

                message_vector.push(received_message);
                let message_vector_clone = message_vector.clone();
                let the_model : Rc<VecModel<SharedString>> =
                        Rc::new(VecModel::from(message_vector_clone.into_iter().map(|messages| messages.into()).collect::<Vec<SharedString>>()));
                let the_model_rc = ModelRc::from(the_model.clone());
            


                    ui.set_messages(the_model_rc);
            }
        }
    });




    // Run the UI loop on the main thread
    ui.run()?;



    Ok(())



}
async fn handle_client(stream: TcpStream){
    
    let websocket = accept_async(stream).await ;
}
