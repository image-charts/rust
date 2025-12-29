//! Example: Download chart as data URI
//!
//! This example shows how to get a chart as a base64-encoded data URI.
//! Useful for embedding directly in HTML or emails.
//! Run with: cargo run --example download_chart_as_data_uri --features async

use image_charts::ImageCharts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_uri = ImageCharts::new()
        .cht("p3")
        .chd("t:60,40")
        .chs("100x100")
        .chl("Hello|World")
        .to_data_uri()
        .await?;

    println!("Data URI length: {} characters", data_uri.len());
    println!("Data URI preview: {}...", &data_uri[..80.min(data_uri.len())]);

    // You can use this in HTML like:
    // <img src="{data_uri}" alt="Chart" />

    Ok(())
}
