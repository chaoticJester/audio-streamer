fn main() {
    let sample_rate = 48000;
    let channels = 2;
    let bytes_per_sample = 2;
    let frequency = 440.0;

    let samples_per_frame = sample_rate / 100;
    let frame_bytes = samples_per_frame * channels * bytes_per_sample;

    println!("Samples per channel in one 10 ms frame: {samples_per_frame}");
    println!("Bytes in one frame: {frame_bytes}");

    for frame_number in 0..3 {
        
        let frame_bytes_vec = build_one_frame(samples_per_frame, frequency, sample_rate, frame_number * samples_per_frame);
        println!("frame_bytes_vec[0] = {}", frame_bytes_vec[0]);
        println!("frame_bytes_vec[1] = {}", frame_bytes_vec[1]);
    }
}

fn build_one_frame(samples_per_frame: u32, frequency: f32, sample_rate: u32, start_sample: u32) -> Vec<u8> {
    let mut samples = Vec::new();
    for n in 0..samples_per_frame {
        let phase = (2.0 * std::f32::consts::PI * frequency * ((start_sample as f32) + (n as f32)) / (sample_rate as f32)) % (sample_rate as f32);
        let sample = (phase.sin() * 10000.0) as i16;
        samples.push(sample);
        samples.push(sample);
    }

    let mut frame_bytes_vec = Vec::new();
    for x in &samples {
        let bytes = x.to_le_bytes();
        frame_bytes_vec.extend_from_slice(&bytes);
    }
    frame_bytes_vec
}
