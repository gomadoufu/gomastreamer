use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "gomastreamer")]
#[command(author = "gomadoufu")]
#[command(version)] // version is automatically detected from Cargo.toml
#[command(about = "gomastreamer:\nThin Rust wrapper for gstreamer, for development, for myself", long_about = None)]
pub struct Cli {
    /// Show information of devices
    #[arg(long)]
    pub show: bool,
    /// Host name of udpsink
    #[arg(default_value = "example.com")]
    pub host: String,
    /// Port number of udpsink
    #[arg(default_value = "5000")]
    pub port: i32,
    /// Source of video
    #[arg(value_enum, short, long, default_value = "test")]
    pub input: Input,
    /// Resolution of video
    #[arg(value_enum, short, long, default_value = "vga")]
    pub resolution: Resolution,
    //フォーマットは将来増えるかもしれないのでブーリアンにしない
    /// Format of video
    #[arg(value_enum, short, long, default_value = "h264")]
    pub format: Format,
    /// Use hardware encode
    #[arg(long = "hardware")]
    pub hardware_encode: bool,
    /// Dry-run mode, just print command
    #[arg(long = "dry-run")]
    pub dry_run: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Input {
    Test,
    Mipi,
    Usb,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Resolution {
    Vga,
    Sd,
    Hd,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Vp8,
    H264,
}
