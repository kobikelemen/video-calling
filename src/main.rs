mod client;
mod audio;






fn main() {
    /* audio */
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();
    let mut aud = audio::Audio::new();
    aud.capture_audio2(&host);
    println!("sample len() after scope ends: {}", aud.clip.samples.len());
    aud.play_audio(&host);


    let server_connection = client::ServerConnection::new();
    let friend_name : String= "jeff".to_string();

    /* send side */
    // let port = 1068; // recv from port
    // let call_connection = client::CallConnection::new(friend_name, server_connection, port);
    // let mut x : Vec<f32> = Vec::new();
    // x.push(69.0);
    // x.push(69.0);
    // x.push(69.0);
    // x.push(69.0);
    // loop {
    //     call_connection.send_data(&x);
    // }


    /* recieve side */
    // let port = 1069; // recv from port
    // let call_connection = client::CallConnection::new(friend_name, server_connection, port);
    // let mut x : Vec<f32> = Vec::new();
    // loop {
    //     call_connection.recv_data();
    // }


    loop {
        match call_connection.recv_data() {
            Some(packet) => {
                // send the packets samples to the sound playing thread using a pipe, 
                //      the thread then adds it into a queue which it plays samples from
            },
            None => {

            },
        }

        // get sound samples from mic and put into packet
        // send packet over network

    }

}
