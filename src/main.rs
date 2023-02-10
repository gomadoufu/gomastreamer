mod exec;
mod parse;
use crate::parse::parse::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    println!("show: {:?}", cli.show);
    println!("host: {:?}", cli.host);
    println!("port: {:?}", cli.port);
    println!("input: {:?}", cli.input);
    println!("resolution: {:?}", cli.resolution);
    println!("format: {:?}", cli.format);
    println!("hardware_encode: {:?}", cli.hardware_encode);
    let gst_launch = exec::exec::Exec::new(
        "gst-launch-1.0".to_string(),
        vec![
            "videotestsrc".to_string(),
            "!".to_string(),
            "autovideosink".to_string(),
        ],
    );
    gst_launch.exec();
}
