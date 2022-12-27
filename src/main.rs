mod client;
mod audio;
mod byte_trates;

use std::sync::mpsc;
use cpal::traits::{HostTrait};


fn run_app<MySampleType, OtherSampleType>(out_device : cpal::Device, inp_device : cpal::Device) 
{
    let (speaker_packet_tx, speaker_packet_rx) = mpsc::channel();
    let (mic_packet_tx, mic_packet_rx) = mpsc::channel();
    let mut aud : audio::Audio<MySampleType,OtherSampleType> = audio::Audio::new(out_device, inp_device, speaker_packet_rx, mic_packet_tx);
    let server_connection = client::ServerConnection::new();
    let friend_name : String = "jeff".to_string();
    let port = 1069; // recv from port
    let call_connection = client::CallConnection::new(friend_name, server_connection, port);
    loop {
        match call_connection.recv_data::<OtherSampleType>() {
            Some(audiopacket) => {
                speaker_packet_tx.send(audiopacket);
            },
            None => {

            },
        }
        if let Ok(audio_packet) = mic_packet_rx.try_recv() {
            // match std::any::TypeId::of(audio_packet.samples[0]) {
            //     std::any::TypeId::of::<f32>() => call_connection.send_data::<f32>(audio_packet),
            //     std::any::TypeId::of::<i16>() => call_connection.send_data::<i16>(audio_packet),
            //     std::any::TypeId::of::<u16>() => call_connection.send_data::<u16>(audio_packet),
            // }
            call_connection.send_data::<MySampleType>(audio_packet);
        }
    }
}


fn main() 
{    
    let host = cpal::default_host();
    let out_device = host.default_output_device().expect("no output device");
    let inp_device = host.default_input_device().expect("no default input device");
    type OtherSampleType = u16; // get from tcp network connection before starting call
    type MySampleType = f32;
    let x = inp_device.supported_input_configs().expect("1").next().expect("2").with_max_sample_rate().sample_format();
    if x == cpal::SampleFormat::F32 {
        run_app::<f32, OtherSampleType>(out_device, inp_device);
    } else if x == cpal::SampleFormat::I16 {
        run_app::<i16, OtherSampleType>(out_device, inp_device);
    } else if x == cpal::SampleFormat::U16 {
        run_app::<u16, OtherSampleType>(out_device, inp_device);
    }

}
