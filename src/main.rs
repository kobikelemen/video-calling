mod client;
mod audio;

use std::sync::mpsc;



fn main() {
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();
    let (playpacket_tx, playpacket_rx) = mpsc::channel();
    let mut aud = audio::Audio::new(&host, playpacket_rx);
    let server_connection = client::ServerConnection::new();
    let friend_name : String= "jeff".to_string();
    let port = 1069; // recv from port
    let call_connection = client::CallConnection::new(friend_name, server_connection, port);

    loop {
        match call_connection.recv_data() {
            Some(audiopacket) => {
                // send the packets samples to the sound playing thread using a pipe, 
                //      the thread then adds it into a queue which it plays samples from
                playpacket_tx.send(audiopacket);
            },
            None => {

            },
        }
        // get sound samples from mic and put into packet
        // send packet over network

    }

}
