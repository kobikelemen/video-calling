extern crate queues;


use crate::byte_traits::{ConvertBytes};
use queues::*;
use std::mem;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};
use cpal::traits::{DeviceTrait};
use std::time::{SystemTime, UNIX_EPOCH};
use std::any::TypeId;
use std::fmt::Display;

// U is other person type, T is my type, S is arbitrary type


// type ClipHandle = Arc<Mutex<Option<(u32, Sender<AudioPacket>, Vec<u8>)>>>;


fn type_size<X>() -> usize
where 
    X: 'static,
{
    if TypeId::of::<X>() == TypeId::of::<f32>() {
        return 4;
    } else if TypeId::of::<X>() == TypeId::of::<i16>() {
        return 2;
    }
    return 2;
    
}


fn convert_sample_type<T,U>(bytes : &Vec<u8>) {
/*
    where 
    T: std::convert::From<f32>,
{
    if TypeId::of::<U>() == TypeId::of::<f32>() {
        let b : [u8; 4] = [0; 4];
        for i in 0..bytes.len() {
            b[i] = bytes[i];
        }
        let x : f32 = f32::from_ne_bytes(b);
        let y : T = x.try_into().expect("FAIL");
        return y;
    }
    } else if TypeId::of::<U>() == TypeId::of::<i16>() {
        let b : [u8; 2] = [0; 2];
        for i in 0..bytes.len() {
            b[i] = bytes[i];
        }
        let x : i16 = i16::from_ne_bytes(b);
        let y : T = x.try_into().expect("FAIL");
        return y;
    } else if TypeId::of::<U>() == TypeId::of::<u16>() {
        let b : [u8; 2] = [0; 2];
        for i in 0..bytes.len() {
            b[i] = bytes[i];
        }
        let x : u16 = u16::from_ne_bytes(b);
        let y : T = x.try_into().expect("FAIL");
        return y;
    }
*/
}



fn add_samples_to_queue<U>(packet_bytes : &Vec<u8>, q_f : &mut Queue<f32>, q_i : &mut Queue<i16>, q_u : &mut Queue<u16>)
where
    U: 'static,
{
    // convert samples from other type to my type then add to queue
    const timesize : usize = 16;
    const seqsize : usize = 4;
    let timest : u128 = u128::from_ne_bytes(packet_bytes[0..timesize].try_into().expect("Failed converting to time format in new_from_bytes()"));
    let seqnum : u32 = u32::from_ne_bytes(packet_bytes[timesize..(timesize+seqsize)].try_into().expect("Failed converting to u32 in new_from_bytes()"));
    let mut i = (timesize + seqsize) as u32;
    let other_samp_size : usize = type_size::<U>();
    while i < packet_bytes.len().try_into().expect("F") {
        if TypeId::of::<U>() == TypeId::of::<f32>() {
            let x : f32 = f32::from_ne_bytes(packet_bytes[((i as usize)-other_samp_size)..(i as usize)].try_into().expect("FAILED"));
            q_f.add(x);
        } else if TypeId::of::<U>() == TypeId::of::<i16>() {
            let x : i16 = i16::from_ne_bytes(packet_bytes[((i as usize)-other_samp_size)..(i as usize)].try_into().expect("FAILED"));
            q_i.add(x);
        } else if TypeId::of::<U>() == TypeId::of::<u16>() {
            let x : u16 = u16::from_ne_bytes(packet_bytes[((i as usize)-other_samp_size)..(i as usize)].try_into().expect("FAILED"));
            q_u.add(x);
        }
        i += other_samp_size as u32;
    }

}


fn write_output_data<T,U>(output: &mut [T], channels: u16, writer: &Arc<Mutex<Option<(usize, Receiver<Vec<u8>>, Queue<f32>, Queue<i16>, Queue<u16>)>>>)
where
    T: cpal::Sample,
    U: Clone + cpal::Sample + 'static,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some((i, audiopacket_rx, que_f, que_i, que_u)) = guard.as_mut() {
            if let Ok(audio_packet_bytes) = audiopacket_rx.try_recv() {
                let my_type = 0; // these need to be passed to this func
                let other_type = 1; 
                add_samples_to_queue::<U>(&audio_packet_bytes, que_f, que_i, que_u);
            }
            for frame in output.chunks_mut(channels.into()) {
                for sample in frame.iter_mut() {
                    if que_f.size() == 0 && que_i.size() == 0 && que_u.size() == 0 {
                        break;
                    }
                    if que_f.size() > 0 {
                        if let Ok(v) = que_f.remove() {
                            println!("PLAYING SAMPLE");
                            *sample = cpal::Sample::from(&v);
                        }
                    } else if que_i.size() > 0 {
                        if let Ok(v) = que_i.remove() {
                            println!("PLAYING SAMPLE");
                            *sample = cpal::Sample::from(&v);
                        }
                    } else if que_u.size() > 0 {
                        if let Ok(v) = que_u.remove() {
                            println!("PLAYING SAMPLE");
                            *sample = cpal::Sample::from(&v);
                        }
                    } else {
                        println!("Failed to remove element from queue");
                    }   
                }
                if que_f.size() == 0 && que_i.size() == 0 && que_u.size() == 0 {
                    break;
                }
            }
        }
    }
}


fn write_input_data<T>(input: &[T], channels: u16, writer: &Arc<Mutex<Option<(u32, Sender<Vec<u8>>, Vec<u8>)>>>)
where
    T: cpal::Sample + ConvertBytes + Display,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some((seq_num, tx, buf)) = guard.as_mut() {
            for frame in input.chunks(channels.into()) {
                let x : T = frame[0].try_into().expect("FAILED");
                let bytes : Vec<u8> = x.to_ne_bytes();
                for b in bytes {
                    buf.push(b);
                }
                if buf.len() == 160 {
                    let mut res : Vec<u8> = Vec::from(seq_num.to_ne_bytes());
                    res.extend_from_slice(&SystemTime::now().duration_since(UNIX_EPOCH).expect("System time failed").as_millis().to_ne_bytes());
                    res.extend_from_slice(&buf);
                    // println!("Sending bytes: {:?}", res);
                    tx.send(res.clone());
                    buf.clear();
                    *seq_num += 1;
                }
            }
        }
    }
    // println!("end of write_input_data()");
}




fn err_fn(e : cpal::StreamError)
{
    println!("error bish");
}

pub struct AudioPacket<S>
{
    pub maxbytes : usize,
    pub timestamp : u128, // use chrono::DateTime::timestamp()
    pub sequencenumber : u32,
    pub samples : Vec<S>,
    pub bytes : Vec<u8>, // incl headers
}

impl<S: ConvertBytes> AudioPacket<S>
{
    pub fn new(time : u128, seqnumber : u32, samps : Vec<S>) -> Self {
        let maxbyte : usize = 160;
        const timesize : usize = 16;
        const seqsize : usize = 4;

        let timestamp_bytes : [u8; timesize] = time.to_ne_bytes();
        let sequencenumber_bytes : [u8; seqsize] = seqnumber.to_ne_bytes();
        let mut bs : Vec<u8> = Vec::new();
        // headers
        for x in 0..timesize {
            bs.push(timestamp_bytes[x]);
        }
        for x in 0..seqsize {
            bs.push(sequencenumber_bytes[x]);
        }
        // fill data bytes
        let lim = maxbyte / mem::size_of::<S>();
        for x in 0..lim {
            let t : Vec<u8> = Vec::from(samps[x].to_ne_bytes());
            for y in t {
                bs.push(y);
            }
        }
        if samps.len() < maxbyte * 4 {
            for x in samps.len()..(maxbyte * 4) {
                bs.push(0);
            }
        }
        Self {
            maxbytes : maxbyte,
            timestamp : time,
            sequencenumber : seqnumber,
            samples : samps,
            bytes : bs,
        }
    }

    /*
    pub fn new_from_bytes(byte : Vec<u8>) -> Self {
        const timesize : usize = 16;
        const seqsize : usize = 4;
        let timest : u128 = u128::from_ne_bytes(byte[0..timesize].try_into().expect("Failed converting to time format in new_from_bytes()"));
        let seqnum : u32 = u32::from_ne_bytes(byte[timesize..(timesize+seqsize)].try_into().expect("Failed converting to u32 in new_from_bytes()"));
        let mut samps : Vec<S> = Vec::new();
        let mut i = timesize + seqsize;
        while i < byte.len() {
            // let s : S = from_bytes::<S>(byte[(i-mem::size_of::<S>())..i].to_vec());
            let s : S = S::from_ne_bytes(byte[(i-mem::size_of::<S>())..i].try_into().expect("Failed converting to sample format in new_from_bytes()"));
            samps.push(s);
            i += mem::size_of::<S>();
        }
        Self {
            maxbytes : 160,
            timestamp : timest,
            sequencenumber : seqnum,
            samples : samps,
            bytes : byte,
        }
    }
    */

    pub fn get_bytes(&self) -> &Vec<u8> {
        return &self.bytes;
    }
}



pub struct AudioClip
{
    pub samples : Vec<f32>,
}


pub struct Audio {
    pub outstream : cpal::Stream,
    pub inpstream : cpal::Stream,
    pub clip : AudioClip,
}


impl Audio {

    pub fn new<T,U>(out_device : cpal::Device , inp_device : cpal::Device, rx : Receiver<Vec<u8>>, tx : Sender<Vec<u8>>) -> Self 
    where 
        T: cpal::Sample + ConvertBytes + Send + Display,
        U: Clone + cpal::Sample + Send + 'static,
    {
        // get sample type, U, that will be received from other user over the network
        // type StateHandle = Arc<Mutex<Option<(usize, Receiver<AudioPacket>, Queue<U>)>>>;
        let q_f : Queue<f32> = Queue::new();
        let q_i : Queue<i16> = Queue::new();
        let q_u : Queue<u16> = Queue::new();
        let num : usize = 0;
        let out_state = (num, rx, q_f, q_i, q_u);
        let out_state = Arc::new(Mutex::new(Some(out_state)));
        let mut out_supported_configs_range = out_device.supported_output_configs().expect("error while querying configs");
        let out_config = out_supported_configs_range.next().expect("no supported config?!").with_max_sample_rate();
        let out_channels = out_config.channels();
        let out_s = match out_config.sample_format() {
            cpal::SampleFormat::F32 => {
                out_device.build_output_stream(
                    &out_config.into(),
                    move |inp_data, _: &_| write_output_data::<f32,U>(inp_data, out_channels, &out_state),
                    err_fn,
                )
            },
            cpal::SampleFormat::I16 => {
                out_device.build_output_stream(
                    &out_config.into(),
                    move |inp_data, _: &_| write_output_data::<i16,U>(inp_data, out_channels, &out_state),
                    err_fn,
                )
            },
            cpal::SampleFormat::U16 => {
                out_device.build_output_stream(
                    &out_config.into(),
                    move |inp_data, _: &_| write_output_data::<u16,U>(inp_data, out_channels, &out_state),
                    err_fn,
                )
            },
        };

        // let inp_device = host.default_input_device().expect("no default input device");
        println!("Input device: {:?}", inp_device.name());
        let mut inp_supported_configs_range = inp_device.supported_input_configs().expect("error while querying configs");
        let inp_config = inp_supported_configs_range.next().expect("no supported config?!").with_max_sample_rate();
        let buffer : Vec<u8> = Vec::new();
        let inp_state = Arc::new(Mutex::new(Some((0, tx, buffer))));
        let inp_channels = inp_config.channels();
        let inp_s = match inp_config.sample_format() {
            cpal::SampleFormat::F32 => inp_device.build_input_stream(
                &inp_config.into(),
                move |inp_data, _: &_| write_input_data::<f32>(inp_data, inp_channels, &inp_state),
                err_fn,
            ),
            cpal::SampleFormat::I16 => inp_device.build_input_stream(
                &inp_config.into(),
                move |inp_data, _: &_| write_input_data::<i16>(inp_data, inp_channels, &inp_state),
                err_fn,
            ),
            cpal::SampleFormat::U16 => inp_device.build_input_stream(
                &inp_config.into(),
                move |inp_data, _: &_| write_input_data::<u16>(inp_data, inp_channels, &inp_state),
                err_fn,
            ),
        };

        Self {
            outstream : out_s.expect("Building output stream FAILED"),
            inpstream : inp_s.expect("Building input stream FAILED"),
            clip : AudioClip {
                samples : Vec::new(),
            },
        }
    }
}