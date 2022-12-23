mod client;
mod audio;

use std::sync::mpsc;


fn main() {
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();
    let (speaker_packet_tx, speaker_packet_rx) = mpsc::channel();
    let (mic_packet_tx, mic_packet_rx) = mpsc::channel();
    let mut aud = audio::Audio::new(&host, speaker_packet_rx, mic_packet_tx);
    let server_connection = client::ServerConnection::new();
    let friend_name : String = "jeff".to_string();
    let port = 1069; // recv from port
    let call_connection = client::CallConnection::new(friend_name, server_connection, port);

    loop {
        match call_connection.recv_data() {
            Some(audiopacket) => {
                speaker_packet_tx.send(audiopacket);
            },
            None => {

            },
        }
        if let Ok(audio_packet) = mic_packet_rx.try_recv() {
            call_connection.send_data(audio_packet)
        }
    }

}
