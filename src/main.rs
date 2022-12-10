use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;

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

    fn capture_audio2(& mut self, host : &cpal::Host) {
        let device = host
            .default_input_device()
            .expect("no default input device");
        
        println!("Input device: {:?}", device.name());
        let mut inp_supported_configs_range = device.supported_input_configs().expect("error while querying configs");
        let config = inp_supported_configs_range.next().expect("no supported config?!").with_max_sample_rate();
        
        let (tx, rx) = mpsc::channel();


        let clipx = AudioClip {
            samples : self.clip.samples.clone(),
        };
        let clip = Arc::new(Mutex::new(Some((clipx, tx))));
        let clip_2 = clip.clone();
        let clip_3 = clip.clone();

        println!("Begin recording...");
        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let channels = config.channels();

        type ClipHandle = Arc<Mutex<Option<(AudioClip, Sender<()>)>>>;


        fn write_input_data<T>(input: &[T], channels: u16, writer: &ClipHandle)
        where
            T: cpal::Sample,
        {
            if let Ok(mut guard) = writer.try_lock() {
                if let Some((clip, tx)) = guard.as_mut() {
                    for frame in input.chunks(channels.into()) {
                        let x = frame[0].to_f32();
                        clip.samples.push(frame[0].to_f32());
                        if clip.samples.len() > 100000 {
                            tx.send(());
                        }
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

        let recv = rx.recv().unwrap();

        // loop {
        //     if let Ok(mut guard) = clip_3.try_lock() {
        //         if let Some(clip_3) = guard.as_mut() {
        //             if clip_3.samples.len() > 100000 {
        //                 break;
        //             }
        //         }
        //     }
        // }

        drop(stream);
        let (clip, tx) = clip_3.lock().unwrap().take().unwrap();
        eprintln!("Recorded {} samples", clip.samples.len());
        self.clip.samples = clip.samples;
    }




    fn play_audio(& mut self, host : &cpal::Host){
        let device = host
            .default_output_device()
            .expect("no output device");
        
        let mut out_supported_configs_range = device.supported_output_configs().expect("error while querying configs");
        let config = out_supported_configs_range.next().expect("no supported config?!").with_max_sample_rate();

        println!("Begin playback...");

        let (tx, rx) = mpsc::channel();

        let state = (0, self.clip.samples.clone(), tx);

        type StateHandle = Arc<Mutex<Option<(usize, Vec<f32>, Sender<()>)>>>;

        let state = Arc::new(Mutex::new(Some(state)));

        let channels = config.channels();

        fn write_output_data<T>(output: &mut [T], channels: u16, writer: &StateHandle)
        where
            T: cpal::Sample,
        {
            if let Ok(mut guard) = writer.try_lock() {
                if let Some((i, clip, donex)) = guard.as_mut() {
                    for frame in output.chunks_mut(channels.into()) {
                        for sample in frame.iter_mut() {
                            *sample = cpal::Sample::from(clip.get(*i).unwrap_or(&0f32));
                        }
                        *i += 1;
                    }
                    if *i >= clip.len() {
                        donex.send(());
                    }
                }
            }
        }

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data, _: &_| write_output_data::<f32>(data, channels, &state),
                err_fn,
            ),
            cpal::SampleFormat::I16 => device.build_output_stream(
                &config.into(),
                move |data, _: &_| write_output_data::<i16>(data, channels, &state),
                err_fn,
            ),
            cpal::SampleFormat::U16 => device.build_output_stream(
                &config.into(),
                move |data, _: &_| write_output_data::<u16>(data, channels, &state),
                err_fn,
            ),
        };

        let rec = rx.recv().unwrap();
        println!("Finished playback");
    }
}





fn main() {
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();

    let mut audio = Audio::new();
    
    audio.capture_audio2(&host);

    println!("sample len() after scope ends: {}", audio.clip.samples.len());

    audio.play_audio(&host);


}
