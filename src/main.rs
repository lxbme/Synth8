use retro_music_maker::{generate_square_signal, create_wav_file_writer, write_samples_to_file};

fn main() {
    let duration_secs = 2;
    let frequency = 440.0;
    let filename = "square_wave.wav";

    
    let samples = match generate_square_signal(duration_secs, frequency) {
        Ok(samples) => samples,
        Err(e) => panic!("Error generating signal: {}", e),
    };
    

    let mut writer = match create_wav_file_writer(filename){
        Ok(writer) => writer,
        Err(e) => panic!("Error creating WAV file writer: {}", e),
    };

    match write_samples_to_file(&mut writer, samples) {
        Ok(_) => println!("Write samples to file successfully"),
        Err(e) => println!("Error writing samples to file: {}", e),
    };

    match writer.finalize() {
        Ok(_) => println!("Finalized WAV file"),
        Err(e) => println!("Error finalizing WAV file: {}", e),
    };

    println!("Generated square_wave.wav");
}
