use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn err_fn(e : cpal::StreamError)
{
    println!("error bish");
}


struct AudioClip
{
    samples : Vec<f32>,
}


struct Audio {
    clip : AudioClip,
}

impl Audio {

    fn new() -> Self {
        Self {
            clip : AudioClip {
                samples : Vec::new(),
            },
        }
    }

    fn capture_audio2(&self, host : &cpal::Host) {
        let device = host
            .default_input_device()
            .expect("no default input device");
        
        println!("Input device: {:?}", device.name());
    
        let mut inp_supported_configs_range = device.supported_input_configs().expect("error while querying configs");
        let config = inp_supported_configs_range.next().expect("no supported config?!").with_max_sample_rate();
        
        let clip = AudioClip {
            samples: Vec::new(),
        };
        let clip = Arc::new(Mutex::new(Some(clip)));
        let clip_2 = clip.clone();

        println!("Begin recording...");
        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let channels = config.channels();

        type ClipHandle = Arc<Mutex<Option<AudioClip>>>;

        fn write_input_data<T>(input: &[T], channels: u16, writer: &ClipHandle)
        where
            T: cpal::Sample,
        {
            if let Ok(mut guard) = writer.try_lock() {
                if let Some(clip) = guard.as_mut() {
                    for frame in input.chunks(channels.into()) {
                        let x = frame[0].to_f32();
                        println!("{x}");
                        clip.samples.push(frame[0].to_f32());
                    }
                }
            }
        }

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<f32>(data, channels, &clip_2),
                err_fn,
            ),
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i16>(data, channels, &clip_2),
                err_fn,
            ),
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<u16>(data, channels, &clip_2),
                err_fn,
            ),
        };
        
        while self.clip.samples.len() < 100 {

        }

        drop(stream);
        let clip = clip.lock().unwrap().take().unwrap();
        eprintln!("Recorded {} samples", clip.samples.len());
    }




    // fn play_audio(&self, host : &cpal::Host){
    //     let device = host
    //         .default_output_device()
    //         .expect("no output device");
    //     println!("Output device: {}", device.name());
    //     let config = device.default_output_config();

    //     println!("Begin playback...");

    //     type StateHandle = Arc<Mutex<Option<(usize, Vec<f32>, Sender<()>)>>>;
    //     let sample_rate = config.sample_rate().0;
    //     // let (done_tx, done_rx) = channel::<()>();
    //     // let state = (0, self.resample(sample_rate).samples, done_tx);
    //     // let state = Arc::new(Mutex::new(Some(state)));
    //     let channels = config.channels();

    //     let err_fn = move |err| {
    //         eprintln!("an error occurred on stream: {}", err);
    //     };

    //     fn write_output_data<T>(output: &mut [T], channels: u16, writer: &StateHandle)
    //     where
    //         T: cpal::Sample,
    //     {
    //         if let Ok(mut guard) = writer.try_lock() {
    //             if let Some((i, clip.samples, done)) = guard.as_mut() {
    //                 for frame in output.chunks_mut(channels.into()) {
    //                     for sample in frame.iter_mut() {
    //                         *sample = cpal::Sample::from(clip.samples.get(*i).unwrap_or(&0f32));
    //                     }
    //                     *i += 1;
    //                 }
    //                 if *i >= clip.samples.len() {
    //                     if let Err(_) = done.send(()) {
    //                         // Playback has already stopped. We'll be dead soon.
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     }

    }





fn main() {
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();

    let mut audio = Audio::new();
    
    audio.capture_audio2(&host);

    // audio.play_audio(&host);


}
