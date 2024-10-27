use hound;
use plotters::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex32};

fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let reader = hound::WavReader::open("waves.wav")?;
    let sample_rate = reader.spec().sample_rate as f32;
    
    let samples: Vec<f32> = reader.into_samples::<i16>()
        .map(|x| x.unwrap() as f32 / i16::MAX as f32)
        .collect();

    println!("Successfully read {} samples at {} Hz", samples.len(), sample_rate);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());
    
    let mut buffer: Vec<Complex32> = samples.iter()
        .map(|&x| Complex32::new(x, 0.0))
        .collect();

    fft.process(&mut buffer);

    let magnitudes: Vec<f32> = buffer.iter()
        .take(buffer.len() / 2)
        .map(|c| (c.norm() / (buffer.len() as f32).sqrt()).log10())
        .collect();

    let root = BitMapBackend::new("fft_output.png", (800, 600))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let max_freq = sample_rate / 2.0;
    let freq_step = max_freq / (magnitudes.len() as f32);

    let min_magnitude = magnitudes.iter()
        .fold(f32::INFINITY, |a, &b| a.min(b));
    let max_magnitude = magnitudes.iter()
        .fold(f32::NEG_INFINITY, |a, &b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("FFT Spectrum", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0f32..max_freq,
            min_magnitude..max_magnitude
        )?;

    chart.configure_mesh()
        .x_desc("Frequency (Hz)")
        .y_desc("Magnitude (dB)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        magnitudes.iter().enumerate()
            .map(|(i, &mag)| (i as f32 * freq_step, mag)),
        &BLUE,
    ))?;

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    println!("FFT plot has been saved as 'fft_output.png'");
    Ok(())
}