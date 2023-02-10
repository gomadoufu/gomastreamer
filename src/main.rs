use clap::Parser;

#[derive(Parser)]
#[command(name = "gomastreamer")]
#[command(author = "gomadoufu")]
#[command(version = "0.1.0")]
#[command(about = "Thin gstreamer wrapper for development, for myself", long_about = None)]
struct Cli {
    #[arg(long)]
    show: bool,
    #[arg(default_value = "http://localhost")]
    host: String,
    #[arg(default_value = "8080")]
    port: i32,
    #[arg(short, long, value_name = "test | mipi | usb", default_value = "test")]
    input: String,
    #[arg(short, long, value_name = "vga | sd | hd", default_value = "vga")]
    resolution: String,
    //フォーマットは将来増えるかもしれないのでブーリアンにしない
    #[arg(short, long, value_name = "vp8 | h264", default_value = "vp8")]
    format: String,
    #[arg(long = "hardware")]
    hardware_encode: bool,
}

fn main() {
    let cli = Cli::parse();
    println!("show: {:?}", cli.show);
    println!("host: {:?}", cli.host);
    println!("port: {:?}", cli.port);
    println!("input: {:?}", cli.input);
    println!("resolution: {:?}", cli.resolution);
    println!("format: {:?}", cli.format);
    println!("hardware_encode: {:?}", cli.hardware_encode);
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
