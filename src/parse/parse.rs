use clap::Parser;

#[derive(Parser)]
#[command(name = "gomastreamer")]
#[command(author = "gomadoufu")]
#[command(version = "0.1.0")]
#[command(about = "Thin gstreamer wrapper for development, for myself", long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub show: bool,
    #[arg(default_value = "http://localhost")]
    pub host: String,
    #[arg(default_value = "8080")]
    pub port: i32,
    #[arg(short, long, value_name = "test | mipi | usb", default_value = "test")]
    pub input: String,
    #[arg(short, long, value_name = "vga | sd | hd", default_value = "vga")]
    pub resolution: String,
    //フォーマットは将来増えるかもしれないのでブーリアンにしない
    #[arg(short, long, value_name = "vp8 | h264", default_value = "vp8")]
    pub format: String,
    #[arg(long = "hardware")]
    pub hardware_encode: bool,
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
