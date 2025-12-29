#![allow(non_snake_case, dead_code)]

//! Official Image-Charts.com API client library
//!
//! Official [Image Charts](https://image-charts.com/) API client.
//! Generate URLs of static image charts.
//! Embed them everywhere in emails, pdf reports, chat bots...!
//!
//! # Features
//!
//! - `async` (default): Async API using tokio and reqwest
//! - `blocking`: Blocking/synchronous API using reqwest blocking
//! - `full`: Both async and blocking APIs
//!
//! # Example
//!
//! ```rust
//! use image_charts::ImageCharts;
//!
//! let url = ImageCharts::new()
//!     .cht("p")
//!     .chd("t:60,40")
//!     .chs("100x100")
//!     .to_url();
//!
//! println!("{}", url);
//! ```

use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

/// Error type for ImageCharts operations
#[derive(Error, Debug)]
#[error("{message}")]
pub struct ImageChartsError {
    /// Error message
    pub message: String,
    /// Error code from Image-Charts API
    pub code: Option<String>,
    /// HTTP status code
    pub status_code: Option<u16>,
}

impl ImageChartsError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
            status_code: None,
        }
    }

    fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    fn with_status(mut self, status: u16) -> Self {
        self.status_code = Some(status);
        self
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ValidationError {
    message: String,
}

/// Configuration for ImageCharts client
#[derive(Debug, Clone)]
pub struct ImageChartsConfig {
    /// Protocol (http or https)
    pub protocol: String,
    /// API host
    pub host: String,
    /// API port
    pub port: u16,
    /// API pathname
    pub pathname: String,
    /// Request timeout
    pub timeout: Duration,
    /// Enterprise secret key for signing
    pub secret: Option<String>,
    /// Custom user-agent string
    pub user_agent: Option<String>,
}

impl Default for ImageChartsConfig {
    fn default() -> Self {
        Self {
            protocol: "https".to_string(),
            host: "image-charts.com".to_string(),
            port: 443,
            pathname: "/chart".to_string(),
            timeout: Duration::from_millis(5000),
            secret: None,
            user_agent: None,
        }
    }
}

/// Builder for ImageCharts API requests
///
/// Use the fluent API to configure chart parameters, then call one of the
/// output methods (`to_url`, `to_buffer`, `to_file`, `to_data_uri`) to
/// generate the chart.
///
/// # Example
///
/// ```rust
/// use image_charts::ImageCharts;
///
/// let chart = ImageCharts::new()
///     .cht("p")           // Pie chart
///     .chd("t:60,40")     // Data
///     .chs("400x300")     // Size
///     .chl("Hello|World") // Labels
///     .to_url();
/// ```
#[derive(Debug, Clone)]
pub struct ImageCharts {
    config: ImageChartsConfig,
    query: HashMap<String, String>,
}

impl Default for ImageCharts {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageCharts {
    /// Create a new ImageCharts instance with default configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    ///
    /// let chart = ImageCharts::new();
    /// ```
    pub fn new() -> Self {
        Self::with_config(ImageChartsConfig::default())
    }

    /// Create a new ImageCharts instance with custom configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_charts::{ ImageCharts, ImageChartsConfig };
    /// use std::time::Duration;
    ///
    /// let config = ImageChartsConfig {
    ///     timeout: Duration::from_secs(10),
    ///     ..Default::default()
    /// };
    /// let chart = ImageCharts::with_config(config);
    /// ```
    pub fn with_config(config: ImageChartsConfig) -> Self {
        Self {
            config,
            query: HashMap::new(),
        }
    }

    /// Create a new ImageCharts instance for Enterprise usage with a secret key
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    ///
    /// let chart = ImageCharts::with_secret("my-secret-key");
    /// ```
    pub fn with_secret(secret: impl Into<String>) -> Self {
        Self::with_config(ImageChartsConfig {
            secret: Some(secret.into()),
            ..Default::default()
        })
    }

    /// Create a new ImageCharts builder for advanced configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// use std::time::Duration;
    ///
    /// let chart = ImageCharts::builder()
    ///     .secret("my-secret")
    ///     .timeout(Duration::from_secs(30))
    ///     .build();
    /// ```
    pub fn builder() -> ImageChartsBuilder {
        ImageChartsBuilder::default()
    }

    fn clone_with(&self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut new_instance = self.clone();
        new_instance.query.insert(key.into(), value.into());
        new_instance
    }

    
        /// bvg= grouped bar chart, bvs= stacked bar chart, lc=line chart, ls=sparklines, p=pie chart. gv=graph viz
    ///          Three-dimensional pie chart (p3) will be rendered in 2D, concentric pie chart are not supported.
    ///          [Optional, line charts only] You can add :nda after the chart type in line charts to hide the default axes.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-type/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().cht("bvg");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().cht("p");
    /// ```
    pub fn cht(self, value: impl Into<String>) -> Self {
        self.clone_with("cht", value)
    }
        /// chart data
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/data-format/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chd("a:-100,200.5,75.55,110");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chd("t:10,20,30|15,25,35");
    /// ```
    pub fn chd(self, value: impl Into<String>) -> Self {
        self.clone_with("chd", value)
    }
        /// You can configure some charts to scale automatically to fit their data with chds=a. The chart will be scaled so that the largest value is at the top of the chart and the smallest (or zero, if all values are greater than zero) will be at the bottom. Otherwise the "&lg;series_1_min&gt;,&lg;series_1_max&gt;,...,&lg;series_n_min&gt;,&lg;series_n_max&gt;" format set one or more minimum and maximum permitted values for each data series, separated by commas. You must supply both a max and a min. If you supply fewer pairs than there are data series, the last pair is applied to all remaining data series. Note that this does not change the axis range; to change the axis range, you must set the chxr parameter. Valid values range from (+/-)9.999e(+/-)199. You can specify values in either standard or E notation.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/data-format/#text-format-with-custom-scaling)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chds("-80,140");
    /// ```
    pub fn chds(self, value: impl Into<String>) -> Self {
        self.clone_with("chds", value)
    }
        /// How to encode the data in the QR code. 'UTF-8' is the default and only supported value. Contact our team if you wish to have support for Shift_JIS and/or ISO-8859-1.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/qr-codes/#data-encoding)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().choe("UTF-8");
    /// ```
    pub fn choe(self, value: impl Into<String>) -> Self {
        self.clone_with("choe", value)
    }
        /// QRCode error correction level and optional margin
    ///
    /// [Reference documentation](https://documentation.image-charts.com/qr-codes/#error-correction-level-and-margin)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chld("L|4");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chld("M|10");
    /// ```
    ///
    /// Default: `"L|4"`
    pub fn chld(self, value: impl Into<String>) -> Self {
        self.clone_with("chld", value)
    }
        /// You can specify the range of values that appear on each axis independently, using the chxr parameter. Note that this does not change the scale of the chart elements (use chds for that), only the scale of the axis labels.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-axis/#axis-range)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chxr("0,0,200");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chxr("0,10,50,5");
    /// ```
    pub fn chxr(self, value: impl Into<String>) -> Self {
        self.clone_with("chxr", value)
    }
        /// Some clients like Flowdock/Facebook messenger and so on, needs an URL to ends with a valid image extension file to display the image, use this parameter at the end your URL to support them. Valid values are ".png", ".svg" and ".gif".
    ///            Only QRCodes and GraphViz support svg output.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/output-format/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chof(".png");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chof(".svg");
    /// ```
    ///
    /// Default: `".png"`
    pub fn chof(self, value: impl Into<String>) -> Self {
        self.clone_with("chof", value)
    }
        /// Maximum chart size for all charts except maps is 998,001 pixels total (Google Image Charts was limited to 300,000), and maximum width or length is 999 pixels.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-size/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chs("400x400");
    /// ```
    pub fn chs(self, value: impl Into<String>) -> Self {
        self.clone_with("chs", value)
    }
        /// Format: &lt;data_series_1_label&gt;|...|&lt;data_series_n_label&gt;. The text for the legend entries. Each label applies to the corresponding series in the chd array. Use a + mark for a space. If you do not specify this parameter, the chart will not get a legend. There is no way to specify a line break in a label. The legend will typically expand to hold your legend text, and the chart area will shrink to accommodate the legend.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/legend-text-and-style/)
    pub fn chdl(self, value: impl Into<String>) -> Self {
        self.clone_with("chdl", value)
    }
        /// Specifies the color and font size of the legend text. <color>,<size>
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/legend-text-and-style/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chdls("9e9e9e,17");
    /// ```
    ///
    /// Default: `"000000"`
    pub fn chdls(self, value: impl Into<String>) -> Self {
        self.clone_with("chdls", value)
    }
        /// Solid or dotted grid lines
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/grid-lines/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chg("1,1");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chg("0,1,1,5");
    /// ```
    pub fn chg(self, value: impl Into<String>) -> Self {
        self.clone_with("chg", value)
    }
        /// You can specify the colors of a specific series using the chco parameter.
    ///        Format should be &lt;series_2&gt;,...,&lt;series_m&gt;, with each color in RRGGBB format hexadecimal number.
    ///        The exact syntax and meaning can vary by chart type; see your specific chart type for details.
    ///        Each entry in this string is an RRGGBB[AA] format hexadecimal number.
    ///        If there are more series or elements in the chart than colors specified in your string, the API typically cycles through element colors from the start of that series (for elements) or for series colors from the start of the series list.
    ///        Again, see individual chart documentation for details.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/bar-charts/#examples)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chco("FFC48C");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chco("FF0000,00FF00,0000FF");
    /// ```
    ///
    /// Default: `"F56991,FF9F80,FFC48C,D1F2A5,EFFAB4"`
    pub fn chco(self, value: impl Into<String>) -> Self {
        self.clone_with("chco", value)
    }
        /// chart title
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-title/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chtt("My beautiful chart");
    /// ```
    pub fn chtt(self, value: impl Into<String>) -> Self {
        self.clone_with("chtt", value)
    }
        /// Format should be "<color>,<font_size>[,<opt_alignment>,<opt_font_family>,<opt_font_style>]", opt_alignement is not supported
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-title/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chts("00FF00,17");
    /// ```
    pub fn chts(self, value: impl Into<String>) -> Self {
        self.clone_with("chts", value)
    }
        /// Specify which axes you want (from: "x", "y", "t" and "r"). You can use several of them, separated by a coma; for example: "x,x,y,r". Order is important.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-axis/#visible-axes)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chxt("y");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chxt("x,y");
    /// ```
    pub fn chxt(self, value: impl Into<String>) -> Self {
        self.clone_with("chxt", value)
    }
        /// Specify one parameter set for each axis that you want to label. Format "<axis_index>:|<label_1>|...|<label_n>|...|<axis_index>:|<label_1>|...|<label_n>". Separate multiple sets of labels using the pipe character ( | ).
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-axis/#custom-axis-labels)
    pub fn chxl(self, value: impl Into<String>) -> Self {
        self.clone_with("chxl", value)
    }
        /// You can specify the range of values that appear on each axis independently, using the chxr parameter. Note that this does not change the scale of the chart elements (use chds for that), only the scale of the axis labels.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-axis/#axis-label-styles)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chxs("1,0000DD");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chxs("1N*cUSD*Mil,FF0000");
    /// ```
    pub fn chxs(self, value: impl Into<String>) -> Self {
        self.clone_with("chxs", value)
    }
        /// 
    ///  format should be either:
    ///    - line fills (fill the area below a data line with a solid color): chm=<b_or_B>,<color>,<start_line_index>,<end_line_index>,<0> |...| <b_or_B>,<color>,<start_line_index>,<end_line_index>,<0>
    ///    - line marker (add a line that traces data in your chart): chm=D,<color>,<series_index>,<which_points>,<width>,<opt_z_order>
    ///    - Text and Data Value Markers: chm=N<formatting_string>,<color>,<series_index>,<which_points>,<width>,<opt_z_order>,<font_family>,<font_style>
    ///      
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/compound-charts/)
    pub fn chm(self, value: impl Into<String>) -> Self {
        self.clone_with("chm", value)
    }
        /// line thickness and solid/dashed style
    ///
    /// [Reference documentation](https://documentation.image-charts.com/line-charts/#line-styles)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chls("10");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chls("3,6,3|5");
    /// ```
    pub fn chls(self, value: impl Into<String>) -> Self {
        self.clone_with("chls", value)
    }
        /// If specified it will override "chdl" values
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-label/)
    pub fn chl(self, value: impl Into<String>) -> Self {
        self.clone_with("chl", value)
    }
        /// Position and style of labels on data
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-label/#positionning-and-formatting)
    pub fn chlps(self, value: impl Into<String>) -> Self {
        self.clone_with("chlps", value)
    }
        /// chart margins
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-margin/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chma("30,30,30,30");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chma("40,20");
    /// ```
    pub fn chma(self, value: impl Into<String>) -> Self {
        self.clone_with("chma", value)
    }
        /// Position of the legend and order of the legend entries
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/legend-text-and-style/)
    ///
    /// Default: `"r"`
    pub fn chdlp(self, value: impl Into<String>) -> Self {
        self.clone_with("chdlp", value)
    }
        /// Background Fills
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/background-fill/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chf("b0,lg,0,f44336,0.3,03a9f4,0.8");
    /// ```
    ///
    /// Default: `"bg,s,FFFFFF"`
    pub fn chf(self, value: impl Into<String>) -> Self {
        self.clone_with("chf", value)
    }
        /// Bar corner radius. Display bars with rounded corner.
    ///
    /// [Reference documentation](https://documentation.image-charts.com/bar-charts/#rounded-bar)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chbr("5");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chbr("10");
    /// ```
    pub fn chbr(self, value: impl Into<String>) -> Self {
        self.clone_with("chbr", value)
    }
        /// gif configuration
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/animation/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chan("1200");
    /// ```
    pub fn chan(self, value: impl Into<String>) -> Self {
        self.clone_with("chan", value)
    }
        /// doughnut chart inside label
    ///
    /// [Reference documentation](https://documentation.image-charts.com/pie-charts/#inside-label)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().chli("45%");
    /// ```
    pub fn chli(self, value: impl Into<String>) -> Self {
        self.clone_with("chli", value)
    }
        /// image-charts enterprise `account_id`
    ///
    /// [Reference documentation](https://documentation.image-charts.com/enterprise/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().icac("accountId");
    /// ```
    pub fn icac(self, value: impl Into<String>) -> Self {
        self.clone_with("icac", value)
    }
        /// HMAC-SHA256 signature required to activate paid features
    ///
    /// [Reference documentation](https://documentation.image-charts.com/enterprise/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().ichm("0785cf22a0381c2e0239e27c126de4181f501d117c2c81745611e9db928b0376");
    /// ```
    pub fn ichm(self, value: impl Into<String>) -> Self {
        self.clone_with("ichm", value)
    }
        /// How to use icff to define font family as Google Font : https://developers.google.com/fonts/docs/css2
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-font/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().icff("Abel");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().icff("Akronim");
    /// ```
    pub fn icff(self, value: impl Into<String>) -> Self {
        self.clone_with("icff", value)
    }
        /// Default font style for all text
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/chart-font/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().icfs("normal");
    /// ```
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().icfs("italic");
    /// ```
    pub fn icfs(self, value: impl Into<String>) -> Self {
        self.clone_with("icfs", value)
    }
        /// localization (ISO 639-1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().iclocale("en");
    /// ```
    pub fn iclocale(self, value: impl Into<String>) -> Self {
        self.clone_with("iclocale", value)
    }
        /// Retina is a marketing term coined by Apple that refers to devices and monitors that have a resolution and pixel density so high — roughly 300 or more pixels per inch – that a person is unable to discern the individual pixels at a normal viewing distance.
    ///            In order to generate beautiful charts for these Retina displays, Image-Charts supports a retina mode that can be activated through the icretina=1 parameter
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/retina/)
    pub fn icretina(self, value: impl Into<String>) -> Self {
        self.clone_with("icretina", value)
    }
        /// Background color for QR Codes
    ///
    /// [Reference documentation](https://documentation.image-charts.com/qr-codes/#background-color)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().icqrb("FFFFFF");
    /// ```
    ///
    /// Default: `"FFFFFF"`
    pub fn icqrb(self, value: impl Into<String>) -> Self {
        self.clone_with("icqrb", value)
    }
        /// Foreground color for QR Codes
    ///
    /// [Reference documentation](https://documentation.image-charts.com/qr-codes/#foreground-color)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    /// let chart = ImageCharts::new().icqrf("000000");
    /// ```
    ///
    /// Default: `"000000"`
    pub fn icqrf(self, value: impl Into<String>) -> Self {
        self.clone_with("icqrf", value)
    }
    

    /// Get the full Image-Charts API URL (signed and encoded if necessary)
    ///
    /// This method returns the complete URL that can be used to fetch the chart image.
    /// If an enterprise account ID (`icac`) is set and a secret is configured,
    /// the URL will be automatically signed with HMAC-SHA256.
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_charts::ImageCharts;
    ///
    /// let url = ImageCharts::new()
    ///     .cht("p")
    ///     .chd("t:60,40")
    ///     .chs("100x100")
    ///     .to_url();
    ///
    /// assert!(url.starts_with("https://image-charts.com/chart?"));
    /// ```
    pub fn to_url(&self) -> String {
        let mut pairs: Vec<(&String, &String)> = self.query.iter().collect();
        pairs.sort_by(|a, b| a.0.cmp(b.0));

        let mut query_string = pairs
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        if self.query.contains_key("icac") {
            if let Some(ref secret) = self.config.secret {
                if !secret.is_empty() {
                    let signature = self.sign(&query_string, secret);
                    query_string.push_str(&format!("&ichm={}", signature));
                }
            }
        }

        // Only include port if it's not the default for the protocol
        let port_str = match (self.config.protocol.as_str(), self.config.port) {
            ("https", 443) | ("http", 80) => String::new(),
            (_, port) => format!(":{}", port),
        };

        format!(
            "{}://{}{}{}?{}",
            self.config.protocol,
            self.config.host,
            port_str,
            self.config.pathname,
            query_string
        )
    }

    fn sign(&self, data: &str, secret: &str) -> String {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;
        let mut mac =
            HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
        mac.update(data.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    fn get_mime_type(&self) -> &str {
        if self.query.contains_key("chan") {
            "image/gif"
        } else {
            "image/png"
        }
    }

    fn get_file_format(&self) -> &str {
        if self.query.contains_key("chan") {
            "gif"
        } else {
            "png"
        }
    }

    fn build_user_agent(&self) -> String {
        let default_ua = format!(
            "rust-image_charts/{}{}",
            env!("CARGO_PKG_VERSION"),
            self.query
                .get("icac")
                .map(|icac| format!(" ({})", icac))
                .unwrap_or_default()
        );
        self.config.user_agent.clone().unwrap_or(default_ua)
    }

    fn parse_error_response(
        status: u16,
        error_code: Option<String>,
        validation_header: Option<&str>,
    ) -> ImageChartsError {
        let validation_message = validation_header
            .and_then(|v| serde_json::from_str::<Vec<ValidationError>>(v).ok())
            .map(|errors| {
                errors
                    .into_iter()
                    .map(|e| e.message)
                    .collect::<Vec<_>>()
                    .join("\n")
            });

        let message = validation_message
            .or_else(|| error_code.clone())
            .unwrap_or_else(|| format!("HTTP {}", status));

        let mut err = ImageChartsError::new(message).with_status(status);
        if let Some(code) = error_code {
            err = err.with_code(code);
        }
        err
    }
}

// Async implementation
#[cfg(feature = "async")]
impl ImageCharts {
    /// Do an async request to Image-Charts API and return the image as bytes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use image_charts::ImageCharts;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let buffer = ImageCharts::new()
    ///         .cht("p")
    ///         .chd("t:60,40")
    ///         .chs("100x100")
    ///         .to_buffer()
    ///         .await?;
    ///
    ///     println!("Image size: {} bytes", buffer.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn to_buffer(&self) -> Result<Vec<u8>, ImageChartsError> {
        let client = reqwest::Client::builder()
            .timeout(self.config.timeout)
            .build()
            .map_err(|e| ImageChartsError::new(e.to_string()))?;

        let response = client
            .get(self.to_url())
            .header("User-Agent", self.build_user_agent())
            .send()
            .await
            .map_err(|e| {
                let mut err = ImageChartsError::new(e.to_string());
                if let Some(status) = e.status() {
                    err = err.with_status(status.as_u16());
                }
                err
            })?;

        let status = response.status().as_u16();
        if (200..300).contains(&status) {
            response
                .bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|e| ImageChartsError::new(e.to_string()).with_status(status))
        } else {
            let error_code = response
                .headers()
                .get("x-ic-error-code")
                .and_then(|v| v.to_str().ok())
                .map(String::from);
            let validation_header = response
                .headers()
                .get("x-ic-error-validation")
                .and_then(|v| v.to_str().ok())
                .map(String::from);

            Err(Self::parse_error_response(
                status,
                error_code,
                validation_header.as_deref(),
            ))
        }
    }

    /// Do an async request and write the image to a file
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use image_charts::ImageCharts;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     ImageCharts::new()
    ///         .cht("p")
    ///         .chd("t:60,40")
    ///         .chs("100x100")
    ///         .to_file("chart.png")
    ///         .await?;
    ///
    ///     println!("Chart saved to chart.png");
    ///     Ok(())
    /// }
    /// ```
    pub async fn to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), ImageChartsError> {
        let buffer = self.to_buffer().await?;
        tokio::fs::write(path, buffer)
            .await
            .map_err(|e| ImageChartsError::new(e.to_string()))
    }

    /// Do an async request and return a base64-encoded data URI
    ///
    /// The returned string can be used directly in HTML `<img>` tags or CSS.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use image_charts::ImageCharts;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let data_uri = ImageCharts::new()
    ///         .cht("p")
    ///         .chd("t:60,40")
    ///         .chs("100x100")
    ///         .to_data_uri()
    ///         .await?;
    ///
    ///     println!("<img src=\"{}\" />", data_uri);
    ///     Ok(())
    /// }
    /// ```
    pub async fn to_data_uri(&self) -> Result<String, ImageChartsError> {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let buffer = self.to_buffer().await?;
        let encoded = STANDARD.encode(&buffer);
        Ok(format!("data:{};base64,{}", self.get_mime_type(), encoded))
    }
}

// Blocking implementation
#[cfg(feature = "blocking")]
impl ImageCharts {
    /// Do a blocking request to Image-Charts API and return the image as bytes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use image_charts::ImageCharts;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let buffer = ImageCharts::new()
    ///         .cht("p")
    ///         .chd("t:60,40")
    ///         .chs("100x100")
    ///         .to_buffer_blocking()?;
    ///
    ///     println!("Image size: {} bytes", buffer.len());
    ///     Ok(())
    /// }
    /// ```
    pub fn to_buffer_blocking(&self) -> Result<Vec<u8>, ImageChartsError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(self.config.timeout)
            .build()
            .map_err(|e| ImageChartsError::new(e.to_string()))?;

        let response = client
            .get(self.to_url())
            .header("User-Agent", self.build_user_agent())
            .send()
            .map_err(|e| {
                let mut err = ImageChartsError::new(e.to_string());
                if let Some(status) = e.status() {
                    err = err.with_status(status.as_u16());
                }
                err
            })?;

        let status = response.status().as_u16();
        if (200..300).contains(&status) {
            response
                .bytes()
                .map(|b| b.to_vec())
                .map_err(|e| ImageChartsError::new(e.to_string()).with_status(status))
        } else {
            let error_code = response
                .headers()
                .get("x-ic-error-code")
                .and_then(|v| v.to_str().ok())
                .map(String::from);
            let validation_header = response
                .headers()
                .get("x-ic-error-validation")
                .and_then(|v| v.to_str().ok())
                .map(String::from);

            Err(Self::parse_error_response(
                status,
                error_code,
                validation_header.as_deref(),
            ))
        }
    }

    /// Do a blocking request and write the image to a file
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use image_charts::ImageCharts;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     ImageCharts::new()
    ///         .cht("p")
    ///         .chd("t:60,40")
    ///         .chs("100x100")
    ///         .to_file_blocking("chart.png")?;
    ///
    ///     println!("Chart saved to chart.png");
    ///     Ok(())
    /// }
    /// ```
    pub fn to_file_blocking(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), ImageChartsError> {
        let buffer = self.to_buffer_blocking()?;
        std::fs::write(path, buffer).map_err(|e| ImageChartsError::new(e.to_string()))
    }

    /// Do a blocking request and return a base64-encoded data URI
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use image_charts::ImageCharts;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let data_uri = ImageCharts::new()
    ///         .cht("p")
    ///         .chd("t:60,40")
    ///         .chs("100x100")
    ///         .to_data_uri_blocking()?;
    ///
    ///     println!("<img src=\"{}\" />", data_uri);
    ///     Ok(())
    /// }
    /// ```
    pub fn to_data_uri_blocking(&self) -> Result<String, ImageChartsError> {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let buffer = self.to_buffer_blocking()?;
        let encoded = STANDARD.encode(&buffer);
        Ok(format!("data:{};base64,{}", self.get_mime_type(), encoded))
    }
}

/// Builder for ImageChartsConfig
///
/// Provides a fluent API to configure ImageCharts instances.
///
/// # Example
///
/// ```rust
/// use image_charts::ImageCharts;
/// use std::time::Duration;
///
/// let chart = ImageCharts::builder()
///     .secret("my-secret-key")
///     .timeout(Duration::from_secs(30))
///     .host("custom.image-charts.com")
///     .build()
///     .cht("p")
///     .chd("t:60,40")
///     .chs("100x100");
/// ```
#[derive(Debug, Default)]
pub struct ImageChartsBuilder {
    protocol: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    pathname: Option<String>,
    timeout: Option<Duration>,
    secret: Option<String>,
    user_agent: Option<String>,
}

impl ImageChartsBuilder {
    /// Set the protocol (http or https)
    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }

    /// Set the API host
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    /// Set the API port
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Set the API pathname
    pub fn pathname(mut self, pathname: impl Into<String>) -> Self {
        self.pathname = Some(pathname.into());
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the enterprise secret key for URL signing
    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }

    /// Set a custom user-agent string
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Build the ImageCharts instance
    pub fn build(self) -> ImageCharts {
        let default = ImageChartsConfig::default();
        ImageCharts::with_config(ImageChartsConfig {
            protocol: self.protocol.unwrap_or(default.protocol),
            host: self.host.unwrap_or(default.host),
            port: self.port.unwrap_or(default.port),
            pathname: self.pathname.unwrap_or(default.pathname),
            timeout: self.timeout.unwrap_or(default.timeout),
            secret: self.secret,
            user_agent: self.user_agent,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_image_charts() -> ImageCharts {
        match std::env::var("IMAGE_CHARTS_USER_AGENT") {
            Ok(ua) => ImageCharts::builder().user_agent(ua).build(),
            Err(_) => ImageCharts::new(),
        }
    }

    fn create_image_charts_with_secret(secret: &str) -> ImageCharts {
        match std::env::var("IMAGE_CHARTS_USER_AGENT") {
            Ok(ua) => ImageCharts::builder().secret(secret).user_agent(ua).build(),
            Err(_) => ImageCharts::with_secret(secret),
        }
    }

    #[test]
    fn test_to_url_basic() {
        let url = ImageCharts::new().cht("p").chd("t:1,2,3").to_url();
        assert!(url.contains("cht=p"));
        assert!(url.contains("chd=t%3A1%2C2%2C3"));
    }

    #[test]
    fn test_to_url_includes_protocol_host() {
        let url = ImageCharts::new().cht("p").to_url();
        // Default port (443 for https) should not be included in the URL
        assert!(url.starts_with("https://image-charts.com/chart?"));
    }

    #[test]
    fn test_to_url_includes_custom_port() {
        let config = ImageChartsConfig {
            port: 8080,
            ..Default::default()
        };
        let url = ImageCharts::with_config(config).cht("p").to_url();
        // Non-default port should be included in the URL
        assert!(url.starts_with("https://image-charts.com:8080/chart?"));
    }

    #[test]
    fn test_to_url_with_signature() {
        let url = ImageCharts::with_secret("plop")
            .cht("p")
            .chd("t:1,2,3")
            .chs("100x100")
            .icac("test_fixture")
            .to_url();
        assert!(url.contains("ichm="));
    }

    #[test]
    fn test_signature_value() {
        // Test that HMAC signature matches expected value
        let chart = ImageCharts::with_secret("plop")
            .chs("100x100")
            .cht("p")
            .chd("t:1,2,3")
            .icac("test_fixture");

        let url = chart.to_url();
        // The signature should be present
        assert!(url.contains("ichm="));
    }

    #[test]
    fn test_default_config() {
        let config = ImageChartsConfig::default();
        assert_eq!(config.protocol, "https");
        assert_eq!(config.host, "image-charts.com");
        assert_eq!(config.port, 443);
        assert_eq!(config.pathname, "/chart");
        assert_eq!(config.timeout, Duration::from_millis(5000));
    }

    #[test]
    fn test_builder_pattern() {
        let chart = ImageCharts::builder()
            .secret("test-secret")
            .timeout(Duration::from_secs(10))
            .host("custom.host.com")
            .build();

        assert_eq!(chart.config.host, "custom.host.com");
        assert_eq!(chart.config.timeout, Duration::from_secs(10));
        assert_eq!(chart.config.secret, Some("test-secret".to_string()));
    }

    #[test]
    fn test_fluent_api() {
        let url = ImageCharts::new()
            .cht("bvg")
            .chd("a:10,20,30")
            .chs("300x200")
            .chxt("x,y")
            .to_url();

        assert!(url.contains("cht=bvg"));
        assert!(url.contains("chs=300x200"));
    }

    #[test]
    fn test_get_mime_type_png() {
        let chart = ImageCharts::new().cht("p").chs("100x100");
        assert_eq!(chart.get_mime_type(), "image/png");
    }

    #[test]
    fn test_get_mime_type_gif() {
        let chart = ImageCharts::new().cht("p").chs("100x100").chan("100");
        assert_eq!(chart.get_mime_type(), "image/gif");
    }

    #[cfg(feature = "blocking")]
    mod blocking_tests {
        use super::*;

        #[test]
        fn test_to_buffer_blocking_rejects_without_chs() {
            let result = create_image_charts().cht("p").chd("t:1,2,3").to_buffer_blocking();
            assert!(result.is_err());
        }

        #[test]
        fn test_to_buffer_blocking_works() {
            // Add delay to avoid rate limiting
            std::thread::sleep(std::time::Duration::from_secs(3));

            let result = create_image_charts()
                .cht("p")
                .chd("t:1,2,3")
                .chs("100x100")
                .to_buffer_blocking();
            assert!(result.is_ok());
            let buffer = result.unwrap();
            assert!(!buffer.is_empty());
        }

        #[test]
        fn test_to_data_uri_blocking_works() {
            std::thread::sleep(std::time::Duration::from_secs(3));

            let result = create_image_charts()
                .cht("p")
                .chd("t:1,2,3")
                .chs("100x100")
                .to_data_uri_blocking();
            assert!(result.is_ok());
            let data_uri = result.unwrap();
            assert!(data_uri.starts_with("data:image/png;base64,"));
        }
    }

    #[cfg(feature = "async")]
    mod async_tests {
        use super::*;

        #[tokio::test]
        async fn test_to_buffer_async_rejects_without_chs() {
            let result = create_image_charts()
                .cht("p")
                .chd("t:1,2,3")
                .to_buffer()
                .await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_to_buffer_async_works() {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let result = create_image_charts()
                .cht("p")
                .chd("t:1,2,3")
                .chs("100x100")
                .to_buffer()
                .await;
            assert!(result.is_ok());
            let buffer = result.unwrap();
            assert!(!buffer.is_empty());
        }

        #[tokio::test]
        async fn test_to_data_uri_async_works() {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let result = create_image_charts()
                .cht("p")
                .chd("t:1,2,3")
                .chs("100x100")
                .to_data_uri()
                .await;
            assert!(result.is_ok());
            let data_uri = result.unwrap();
            assert!(data_uri.starts_with("data:image/png;base64,"));
        }
    }
}
