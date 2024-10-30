extern crate hound;

use std::f32::consts::PI;

const SAMPLE_RATE: u32 = 44100;  // Standard audio sample rate (44.1 kHz)
const DURATION_SECS: u32 = 5;    // Duration of the audio in seconds
const FREQUENCY: f32 = 90.0;    // Frequency of the sine wave (90 Hz)
const AMPLITUDE: i16 = 3000;     // Amplitude (controls the volume, lower = quieter)

fn main() {
    // WAV file writer configuration
    let spec = hound::WavSpec {
        channels: 1,             // Mono audio
        sample_rate: SAMPLE_RATE, // Sample rate in Hz
        bits_per_sample: 16,      // 16-bit samples
        sample_format: hound::SampleFormat::Int,
    };

    let path = "sine_wave_90hz.wav";
    let mut writer = hound::WavWriter::create(path, spec).expect("Failed to create WAV file");

    // Generate sine wave samples
    for t in 0..(SAMPLE_RATE * DURATION_SECS) {
        let time = t as f32 / SAMPLE_RATE as f32;
        let sample = (AMPLITUDE as f32 * (2.0 * PI * FREQUENCY * time).sin()) as i16;
        writer.write_sample(sample).expect("Failed to write sample");
    }

    // Finalize the file
    writer.finalize().expect("Failed to finalize the WAV file");

    println!("Sine wave file 'sine_wave_256hz.wav' generated successfully.");
}
