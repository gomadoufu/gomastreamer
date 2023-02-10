use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "gomastreamer")]
#[command(author = "gomadoufu")]
#[command(version = "0.1.0")]
#[command(about = "gomastreamer:\nThin gstreamer wrapper for development, for myself", long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub show: bool,
    #[arg(default_value = "http://localhost")]
    pub host: String,
    #[arg(default_value = "8080")]
    pub port: i32,
    #[arg(value_enum, short, long, default_value = "test")]
    pub input: Input,
    #[arg(value_enum, short, long, default_value = "vga")]
    pub resolution: Resolution,
    //フォーマットは将来増えるかもしれないのでブーリアンにしない
    #[arg(value_enum, short, long, default_value = "vp8")]
    pub format: Format,
    #[arg(long = "hardware")]
    pub hardware_encode: bool,
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
