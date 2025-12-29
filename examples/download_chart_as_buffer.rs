//! Example: Download chart as binary buffer
//!
//! This example shows how to download a chart image as bytes.
//! Run with: cargo run --example download_chart_as_buffer --features async

use image_charts::ImageCharts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buffer = ImageCharts::new()
        .cht("p3")
        .chd("t:60,40")
        .chs("700x300")
        .chl("Hello|World")
        .to_buffer()
        .await?;

    println!("Downloaded {} bytes", buffer.len());
    println!("First 20 bytes (PNG header): {:?}", &buffer[..20.min(buffer.len())]);

    Ok(())
}
