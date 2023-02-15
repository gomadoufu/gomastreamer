mod executor;
mod parser;
use crate::executor::exec::Exec;
use crate::parser::{convert::ArgConverter, parse::Cli};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let is_dry_run = cli.dry_run;
    if cli.show {
        let gst_plugins = Exec::new("gst-inspect-1.0".to_string(), vec![]);
        let v4l2_ctl = Exec::new("v4l2-ctl --list-devices".to_string(), vec![]);
        if is_dry_run {
            println!("{}\n{}", "gst-inspect-1.0", "v4l2_ctl --list-devices");
            return;
        }
        println!("Available Devices...");
        v4l2_ctl.exec();
        println!("GStreamer plugins...");
        gst_plugins.exec();
        return;
    }
    let converter = ArgConverter::new();
    let result = converter.convert(cli);
    if is_dry_run {
        println!("gst-launch-1.0 {}", result.join(" "));
        return;
    }
    println!("Running: gst-launch-1.0 {}\t", result.join(" "));
    let gst_launch = Exec::new("gst-launch-1.0".to_string(), result);
    gst_launch.exec();
}
