use libpulse_binding as pulse;
use libpulse_binding::def::BufferAttr;
use libpulse_simple_binding as psimple;
use opus::Application::Voip;
use opus::{Channels, Decoder, Encoder};
use pulse::sample::{Format, Spec};
use std::fs::OpenOptions;
use std::io::Write;
use std::slice;

use crate::implementations::pulseaudio::{PulseAudioConsumer, PulseAudioProducer};
use rand::prelude::*;

mod implementations;
const SAMPLE_RATE: u32 = 48000;
const CHANNELS: usize = 2;
const BUF_SIZE: u32 = 3840; // 20ms of stereo 48kHz 16-bit audio = 48000 samples/sec * 0.02 sec * 2 channels * 2 bytes/sample = 3840 bytes
const FRAME_SIZE: usize = 960; // for opus - 20ms at 48kHz. Per channel, so total samples = FRAME_SIZE * CHANNELS = 1920

#[derive(Debug)]
enum ErrorKind {
    InitializationError,
    WriteError,
    ReadError,
}
trait AudioProducer {
    fn produce(&mut self, data: &mut [u8]) -> Result<(), ErrorKind>;
}

trait Consumer {
    fn consume(&mut self, data: &[u8]) -> Result<usize, ErrorKind>;
}

struct FileConsumer {
    file: std::fs::File,
}

impl FileConsumer {
    fn new(file: &str) -> Result<Self, ErrorKind> {
        match OpenOptions::new().create(true).append(true).open(file) {
            Ok(f) => Ok(FileConsumer { file: f }),
            Err(_) => Err(ErrorKind::WriteError),
        }
    }
}

impl Consumer for FileConsumer {
    fn consume(&mut self, data: &[u8]) -> Result<usize, ErrorKind> {
        match self.file.write(data) {
            Ok(bytes_written) => Ok(bytes_written),
            Err(_) => Err(ErrorKind::WriteError),
        }
    }
}

//mod external;
fn main() {
    let mut client = false;
    let mut server = true;
    for arg in std::env::args() {
        if arg == "--client" {
            client = true;
        }
        if arg == "--server" {
            server = true;
        }
    }
    if client && server {
        eprintln!("Cannot be both client and server");
        return;
    }
    if server {
        send_audio();
    } else if client {
        receive_audio();
    } else {
        eprintln!("Must specify either --client or --server");
    }
}

fn receive_audio() {
    let audio_consumer = PulseAudioConsumer::new().unwrap();
}

fn send_audio() {
    // Can be opened with audacity as raw file, signed 16 bit PCM, 44100 Hz, stereo
    //let mut file_consumer = FileConsumer::new("output.pcm").unwrap();
    //let mut audio_consumer = PulseAudioConsumer::new().unwrap();

    let mut audio_producer = PulseAudioProducer::new().unwrap();
    let consumers: &mut [&mut dyn Consumer] = &mut [];
    let mut data = vec![0u8; BUF_SIZE as usize];
    let mut encoded_data = [0u8; BUF_SIZE as usize];
    let mut decoded_data = vec![0i16; FRAME_SIZE * CHANNELS];
    let mut encoder = opus_encoder();
    let mut decoder = opus_decoder();
    // Get an RNG:
    let mut rng = rand::rng();
    loop {
        match audio_producer.produce(&mut data) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error reading from stream");
                break;
            }
        }

        let pcm: &[i16] =
            unsafe { slice::from_raw_parts(data.as_ptr() as *const i16, data.len() / 2) };

        let samples_needed = FRAME_SIZE * CHANNELS;
        let pcm = &pcm[..samples_needed];
        let n = encoder.encode(&pcm, &mut encoded_data).unwrap();
        if (rng.random_range(0..100)) < 0 {
            eprintln!("Simulating packet loss");
            continue;
        }
        let b = decoder
            .decode(&encoded_data[..n], &mut decoded_data, false)
            .unwrap();

        println!(
            "Read {} samples, data has {} samples, encoded to {} bytes, decoded to {} samples",
            pcm.len(),
            data.len() / 2,
            n,
            b
        );
        consumers.iter_mut().for_each(|c| {
            match c.consume(unsafe {
                slice::from_raw_parts(
                    decoded_data.as_ptr() as *const u8,
                    b * CHANNELS * std::mem::size_of::<i16>(),
                )
            }) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error consuming data: {:?}", e);
                }
            }
        });
    }
}

fn opus_encoder() -> Encoder {
    Encoder::new(SAMPLE_RATE, Channels::Stereo, Voip).unwrap()
}
fn opus_decoder() -> Decoder {
    Decoder::new(SAMPLE_RATE, Channels::Stereo).unwrap()
}
