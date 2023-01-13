mod client;
mod audio;
mod byte_traits;

use std::sync::mpsc;
use cpal::traits::{HostTrait, DeviceTrait};
use byte_traits::ConvertBytes;
use std::fmt::Display;
use std::net::{TcpListener, TcpStream, IpAddr, Ipv4Addr};
use client::{ServerConnection, CallConnectionUDP, wait_for_call, start_call};



fn run_app<MySampleType, OtherSampleType>(out_device : cpal::Device, inp_device : cpal::Device, upscale_factor : u32) 
where
    MySampleType: ConvertBytes + cpal::Sample + Send + Display,
    OtherSampleType: ConvertBytes + Send + cpal::Sample + Clone + 'static,
{
    let (speaker_packet_tx, speaker_packet_rx) = mpsc::channel();
    let (mic_packet_tx, mic_packet_rx) = mpsc::channel();
    let mut aud : audio::Audio = audio::Audio::new::<MySampleType,OtherSampleType>(out_device, inp_device, speaker_packet_rx, mic_packet_tx, upscale_factor);
    let server_connection = client::ServerConnection::new();
    let friend_name : String = "jeff".to_string();
    let my_ip = IpAddr::V4(Ipv4Addr::new(192,168,68,109));
    let my_tcp_port = 1069; // recv from port
    let (other_ip, other_tcp_port) = server_connection.get_friend_addr(friend_name);
    let mut my_udp_port = 1001;
    let call_connection = wait_for_call(other_ip, other_tcp_port, my_ip, my_udp_port, server_connection);
    /* Udp voice call */
    loop {
        match call_connection.recv_data::<OtherSampleType>() {
            Some(audiopacket) => {
                speaker_packet_tx.send(audiopacket);
            },
            None => {
            },
        }
        if let Ok(audio_packet_bytes) = mic_packet_rx.try_recv() {
            call_connection.send_data::<MySampleType>(audio_packet_bytes);
        }
    }
}

// mac is f32, linux is i16


fn main() 
{    
    type OtherSampleType = i16; // get from tcp network connection before starting call
    let host = cpal::default_host();
    let out_device = host.default_output_device().expect("no output device");
    let inp_device = host.default_input_device().expect("no default input device");
    let configs = inp_device.supported_input_configs();
    let my_config = configs.expect("1").next().expect("2").with_max_sample_rate();
    let sample_format = my_config.sample_format();
    let sample_rate : u32 = my_config.sample_rate().0; // frequency
    let transmit_rate : u32 = 2000; // frequency
    let upscale_factor = sample_rate / transmit_rate;
    if sample_format == cpal::SampleFormat::F32 {
        run_app::<f32, OtherSampleType>(out_device, inp_device, upscale_factor);
    } else if sample_format == cpal::SampleFormat::I16 {
        run_app::<i16, OtherSampleType>(out_device, inp_device, upscale_factor);
    } else if sample_format == cpal::SampleFormat::U16 {
        run_app::<u16, OtherSampleType>(out_device, inp_device, upscale_factor);
    }

}
