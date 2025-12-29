//! Example: Enterprise - Sign chart URL with HMAC
//!
//! This example shows how to sign chart URLs for enterprise accounts.
//! The signature ensures the chart URL cannot be tampered with.
//! Run with: cargo run --example enterprise_sign_chart

use image_charts::ImageCharts;

fn main() {
    // Replace with your actual enterprise credentials
    let account_id = "YOUR_ACCOUNT_ID";
    let secret_key = "YOUR_SECRET_KEY";

    let url = ImageCharts::builder()
        .secret(secret_key)
        .build()
        .cht("p3")
        .chd("t:60,40")
        .chs("700x300")
        .chl("Hello|World")
        .icac(account_id)
        .to_url();

    println!("Signed Chart URL: {}", url);

    // The URL will contain an 'ichm' parameter with the HMAC signature
    assert!(url.contains("ichm="), "URL should contain HMAC signature");
    println!("✓ URL is signed with HMAC-SHA256");
}
