use std::fmt::Display;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "gomastreamer")]
#[command(author = "gomadoufu")]
#[command(version)] // version is automatically detected from Cargo.toml
#[command(about = "gomastreamer:\nThin wrapper for gstreamer, for development", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub input: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// INPUT. For help with OUTPUT, run 'gomast input output --help'
    Input(InputArgs),
}

#[derive(Args, Debug)]
pub struct InputArgs {
    #[command(subcommand)]
    pub output: OutputCommands,
    #[arg(value_enum)]
    pub input_type: InputType,
    #[arg(value_enum)]
    pub format: InputFormat,
}

#[derive(Subcommand, Debug)]
pub enum OutputCommands {
    /// output
    Output(OutputArgs),
}

#[derive(Args, Debug)]
pub struct OutputArgs {
    pub width: i32,
    pub height: i32,
    #[arg(value_enum)]
    pub format: OutputFormat,
    /// Host name of udpsink
    pub host: String,
    /// Port number of udpsink
    pub port: i32,
    /// Show information of devices
    #[arg(long)]
    pub show: bool,
    /// Resolution of video
    #[arg(long = "hardware")]
    pub hardware_encode: bool,
    /// Dry-run mode, just print command
    #[arg(long = "dry-run")]
    pub dry_run: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum InputType {
    Test,
    Mipi,
    Usb,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum InputFormat {
    Yuy2,
    Mjpeg,
    Nv12,
    Uyvy,
    H264,
}

impl Display for InputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputFormat::Yuy2 => write!(f, "video/x-raw,format=YUY2"),
            InputFormat::Mjpeg => write!(f, "image/jpeg"),
            InputFormat::Nv12 => write!(f, "video/x-raw,format=NV12"),
            InputFormat::Uyvy => write!(f, "video/x-raw,format=UYVY"),
            InputFormat::H264 => write!(f, "video/x-h264"),
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Vp8,
    H264,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
