mod executor;
mod parser;
use crate::executor::exec::Exec;
use crate::parser::{convert::ArgConverter, parse::Cli};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    if cli.show {
        println!("Showing devices...");
        let gst_device = Exec::new("gst-inspect-1.0".to_string(), vec![]);
        gst_device.exec();
        return;
    }
    let converter = ArgConverter::new();
    let result = converter.convert(cli);
    println!("Running: gst-launch-1.0 {}\t", result.join(" "));
    let gst_launch = Exec::new("gst-launch-1.0".to_string(), result);
    gst_launch.exec();
}
