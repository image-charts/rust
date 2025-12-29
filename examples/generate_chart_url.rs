//! Example: Generate a chart URL
//!
//! This example shows how to generate a chart URL without making any HTTP request.
//! Run with: cargo run --example generate_chart_url

use image_charts::ImageCharts;

fn main() {
    // Create a simple pie chart URL
    let url = ImageCharts::new()
        .cht("p3")
        .chd("t:60,40")
        .chs("700x300")
        .chl("Hello|World")
        .to_url();

    println!("Chart URL: {}", url);

    // Create a more complex bar chart
    let bar_url = ImageCharts::new()
        .cht("bvs")
        .chd("a:30010,-30000,50000,80000,20000")
        .chdl("Income")
        .chf("b0,lg,90,EA469EFF,1,03A9F47C,0.4")
        .chl("2014|2015|2016|2017|2018")
        .chs("700x300")
        .chxt("y")
        .to_url();

    println!("Bar Chart URL: {}", bar_url);
}
