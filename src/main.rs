mod executor;
mod parser;
use crate::{
    executor::{build::Builder, exec::Exec},
    parser::{destruct::Elements, parse::Cli},
};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let destructed = Elements::destruct(cli);
    let dry_run = destructed.dry_run;
    if destructed.show {
        let gst_plugins = Exec::new("gst-inspect-1.0".to_string(), vec![]);
        let v4l2_ctl = Exec::new("v4l2-ctl --list-devices".to_string(), vec![]);
        if dry_run {
            println!("gst-inspect-1.0 \n v4l2_ctl --list-devices");
            return;
        }
        println!("Available Devices...");
        v4l2_ctl.exec();
        println!("GStreamer plugins...");
        gst_plugins.exec();
        return;
    }
    let builder = Builder::new(destructed);
    let result = builder.build();
    if dry_run {
        println!("gst-launch-1.0 {}", result.join(" "));
        return;
    }
    println!("Running: gst-launch-1.0 {}\t", result.join(" "));
    let gst_launch = Exec::new("gst-launch-1.0".to_string(), result);
    gst_launch.exec();
}
