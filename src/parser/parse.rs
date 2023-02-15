use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "gomastreamer")]
#[command(author = "gomadoufu")]
#[command(version)] // version is automatically detected from Cargo.toml
#[command(about = "gomastreamer:\nThin Rust wrapper for gstreamer, for development, for myself", long_about = None)]
pub struct Cli {
    /// show information of devices
    #[arg(long)]
    pub show: bool,
    /// host name of udpsink
    pub host: String,
    /// port number of udpsink
    pub port: i32,
    /// source of video
    #[arg(value_enum, short, long, default_value = "test")]
    pub input: Input,
    /// resolution of video
    #[arg(value_enum, short, long, default_value = "vga")]
    pub resolution: Resolution,
    //フォーマットは将来増えるかもしれないのでブーリアンにしない
    /// format of video
    #[arg(value_enum, short, long, default_value = "vp8")]
    pub format: Format,
    /// use hardware encode
    #[arg(long = "hardware")]
    pub hardware_encode: bool,
    /// dry-run mode
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
