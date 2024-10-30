use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area of 800x600 pixels in a file called "parabola.png"
    let root_area = BitMapBackend::new("parabola.png", (800, 600)).into_drawing_area();
    root_area.fill(&WHITE)?;

    // Set up chart builder, with labels on the axes
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Parabola y = x^2", ("sans-serif", 40).into_font()) // Chart title
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-3f32..3f32, 0f32..10f32)?;  // Smaller X and Y range

    // Configure the mesh (grid) and axis labels
    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .disable_mesh()
        .draw()?;

    // Plot the parabola y = x^2
    chart.draw_series(LineSeries::new(
        (-30..=30).map(|x| {
            let x = x as f32 / 10.0;  // This gives x values between -3.0 and 3.0
            (x, x * x)  // y = x^2
        }),
        &BLUE,
    ))?;

    // Save the chart to the file
    root_area.present()?;

    println!("Plot has been saved to 'parabola.png'");
    Ok(())
}
