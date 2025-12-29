//! Example: Download and save chart as image file
//!
//! This example shows how to download a chart and save it to a file.
//! Run with: cargo run --example download_chart_as_image --features async

use image_charts::ImageCharts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "chart_example.png";

    ImageCharts::new()
        .cht("p3")
        .chd("t:60,40")
        .chs("700x300")
        .chl("Hello|World")
        .to_file(path)
        .await?;

    println!("Chart saved to: {}", path);

    // Clean up
    std::fs::remove_file(path)?;
    println!("File cleaned up");

    Ok(())
}
