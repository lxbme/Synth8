use std::{fmt::Error, fs::File, io::BufWriter};

use dasp::{signal, Signal};
use hound;


const SAMPLE_RATE: u32 = 44100;

pub fn generate_square_signal(duration_secs: u32, frequency: f64) -> Result<Vec<i8>, Error> {
    let mut signal = signal::rate(SAMPLE_RATE as f64)
        .const_hz(frequency)
        .square();

    let mut samples = Vec::new();
    for _ in 0..SAMPLE_RATE * duration_secs {
        let sample = signal.next();
        let sample = (sample * i16::MAX as f64) as i8;
        samples.push(sample);
    }

    Ok(samples)
}

pub fn create_wav_file_writer(filename: &str) -> Result<hound::WavWriter<BufWriter<File>>,hound::Error>{
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 8,
        sample_format: hound::SampleFormat::Int,
    };
    let writer = hound::WavWriter::create(filename, spec)?;
    
    Ok(writer)
}


pub fn write_samples_to_file(writer: &mut hound::WavWriter<BufWriter<File>>, samples: Vec<i8>) -> Result<(),hound::Error> {
    for s in samples.iter() {
        writer.write_sample(*s)?;
    }

    Ok(())
}