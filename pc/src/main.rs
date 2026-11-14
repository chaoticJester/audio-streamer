fn main() {
    let sample_rate = 48000;
    let channels = 2;
    let bytes_per_sample = 2;
    let frequency = 440.0;

    let samples_per_frame = sample_rate / 100;
    let frame_bytes = samples_per_frame * channels * bytes_per_sample;

    println!("Samples per channel in one 10 ms frame: {samples_per_frame}");
    println!("Bytes in one frame: {frame_bytes}");

    let mut samples = Vec::new();
    for n in 0..samples_per_frame {
        let formula = 2.0 * std::f32::consts::PI * frequency * (n as f32) / (sample_rate as f32);
        let sample = (formula.sin() * 10000.0) as i16;
        samples.push(sample);
        samples.push(sample);
    }

    let mut frame_bytes_vec = Vec::new();
    for x in &samples {
        let bytes = x.to_le_bytes();
        frame_bytes_vec.extend_from_slice(&bytes);
    }

    println!("frame_bytes_vec[4] = {}", frame_bytes_vec[4]);
    println!("frame_bytes_vec[5] = {}", frame_bytes_vec[5]);
    
}
