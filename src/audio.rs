extern crate queues;


use crate::byte_trates::{ConvertBytes, from_bytes};
use queues::*;
use std::mem;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};
use cpal::traits::{DeviceTrait};
use std::time::{SystemTime, UNIX_EPOCH};


// U is other person type, T is my type, S is arbitrary type


// type ClipHandle = Arc<Mutex<Option<(u32, Sender<AudioPacket>, Vec<u8>)>>>;







fn write_output_data<T,U>(output: &mut [T], channels: u16, writer: &Arc<Mutex<Option<(usize, Receiver<AudioPacket<U>>, Queue<U>)>>>)
where
    T: cpal::Sample,
    U: Clone + cpal::Sample,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some((i, audiopacket_rx, que)) = guard.as_mut() {
            if let Ok(audio_packet) = audiopacket_rx.try_recv() {
                for x in audio_packet.samples {
                    que.add(x);
                }
            }
            for frame in output.chunks_mut(channels.into()) {
                for sample in frame.iter_mut() {
                    if que.size() == 0 {
                        break;
                    }
                    if let Ok(v) = que.remove() {
                        // convert 'v' of type U to type T before adding it to *sample
                        *sample = cpal::Sample::from(&v);
                    } else {
                        println!("Failed to remove element from queue");
                    }
                }
                if que.size() == 0 {
                    break;
                }
            }
        }
    }
}


fn write_input_data<T>(input: &[T], channels: u16, writer: &Arc<Mutex<Option<(u32, Sender<AudioPacket<T>>, Vec<u8>)>>>)
where
    T: cpal::Sample + ConvertBytes,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some((seq_num, tx, buf)) = guard.as_mut() {
            for frame in input.chunks(channels.into()) {
                let x : T = frame[0].try_into().expect("FAILED");
                for b in x.to_ne_bytes().iter() {
                    buf.push(*b);
                }
                //     let x = frame[0].to_f32();
                //     for b in T::to_ne_bytes(x) {
                //         buf.push(b);
                //     }
                // } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i16>() {
                //     let x = frame[0].to_i16();
                //     for b in T::to_ne_bytes(x) {
                //         buf.push(b);
                //     }
                // } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u16>() {
                //     let x = frame[0].to_u16();
                //     for b in T::to_ne_bytes(x) {
                //         buf.push(b);
                //     }
                // }
                
                if buf.len() == 160 {
                    let mut res : Vec<u8> = Vec::from(seq_num.to_ne_bytes());
                    res.extend_from_slice(&SystemTime::now().duration_since(UNIX_EPOCH).expect("System time failed").as_millis().to_ne_bytes());
                    res.extend_from_slice(&buf);
                    let audiopacket : AudioPacket<T> = AudioPacket::new_from_bytes(res);
                    tx.send(audiopacket);
                    buf.clear();
                    *seq_num += 1;
                }
            }
        }
    }
}




fn err_fn(e : cpal::StreamError)
{
    println!("error bish");
}

pub struct AudioPacket<S>
{
    // pub data_type : u8,
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
        // if samps.len() > (maxbyte / mem::sizeof(T)).try_into().unwrap() {
        //     lim = maxbyte / mem::sizeof(T);
        // } else {
        //     lim = samps.len();
        // }
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

    pub fn new_from_bytes(byte : Vec<u8>) -> Self {
        const timesize : usize = 16;
        const seqsize : usize = 4;
        let timest : u128 = u128::from_ne_bytes(byte[0..timesize].try_into().expect("Failed converting to time format in new_from_bytes()"));
        let seqnum : u32 = u32::from_ne_bytes(byte[timesize..(timesize+seqsize)].try_into().expect("Failed converting to u32 in new_from_bytes()"));
        let mut samps : Vec<S> = Vec::new();
        let mut i = timesize + seqsize;
        while i < byte.len() {
            // match std::any::TypeId<S>() {
            //     std::any::TypeId::of::<f32>() => {
            //         let s : f32 = f32::from_ne_bytes(byte[(i-mem::size_of::<f32>())..i]);

            //     },
            // }
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

    pub fn new<T,U>(out_device : cpal::Device , inp_device : cpal::Device, host : &cpal::Host, rx : Receiver<AudioPacket<U>>, tx : Sender<AudioPacket<T>>) -> Self 
    where 
        T: cpal::Sample + ConvertBytes + Send,
        U: Clone + cpal::Sample + Send,
    {
        // get sample type, U, that will be received from other user over the network
        // type StateHandle = Arc<Mutex<Option<(usize, Receiver<AudioPacket>, Queue<U>)>>>;
        let q : Queue<U> = Queue::new();
        let mut out_supported_configs_range = out_device.supported_output_configs().expect("error while querying configs");
        let out_config = out_supported_configs_range.next().expect("no supported config?!").with_max_sample_rate();
        let num : usize = 0;
        let out_state = (num, rx, q);
        let out_state = Arc::new(Mutex::new(Some(out_state)));
        let out_channels = out_config.channels();
        let out_s = match out_config.sample_format() {
            cpal::SampleFormat::F32 => out_device.build_output_stream(
                &out_config.into(),
                move |inp_data, _: &_| write_output_data::<f32,U>(inp_data, out_channels, &out_state),
                err_fn,
            ),
            cpal::SampleFormat::I16 => out_device.build_output_stream(
                &out_config.into(),
                move |inp_data, _: &_| write_output_data::<i16,U>(inp_data, out_channels, &out_state),
                err_fn,
            ),
            cpal::SampleFormat::U16 => out_device.build_output_stream(
                &out_config.into(),
                move |inp_data, _: &_| write_output_data::<u16,U>(inp_data, out_channels, &out_state),
                err_fn,
            ),
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
                move |inp_data, _: &_| write_input_data::<T>(inp_data, inp_channels, &inp_state),
                err_fn,
            ),
            cpal::SampleFormat::I16 => inp_device.build_input_stream(
                &inp_config.into(),
                move |inp_data, _: &_| write_input_data::<T>(inp_data, inp_channels, &inp_state),
                err_fn,
            ),
            cpal::SampleFormat::U16 => inp_device.build_input_stream(
                &inp_config.into(),
                move |inp_data, _: &_| write_input_data::<T>(inp_data, inp_channels, &inp_state),
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


    

    // pub fn capture_audio2(& mut self, host : &cpal::Host, tx : Sender<AudioPacket>) {
        
    // }


    pub fn play_audio(& mut self, host : &cpal::Host, rx : Receiver<()>){
    
    }
}