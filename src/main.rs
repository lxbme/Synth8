use dasp::{signal, Signal};
use hound;


const SAMPLE_RATE: u32 = 44100;

fn main() {
    let duration_secs = 2;
    let frequency = 440.0;
    let amplitude = 0.3;

    // 生成方波信号
    let mut signal = signal::rate(SAMPLE_RATE as f64)
        .const_hz(frequency)
        .square();
        //.amplify(amplitude);

    // 创建WAV文件
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("square_wave.wav", spec).unwrap();

    for _ in 0..SAMPLE_RATE * duration_secs {
        let sample = signal.next();
        let sample = (sample * i16::MAX as f64) as i16;
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();

    println!("Generated square_wave.wav");
}
