mod exec;
mod parser;
use crate::exec::exec::Exec;
use crate::parser::{convert::ArgConverter, parse::Cli};
use clap::Parser;
use parser::convert::MockCli;

fn main() {
    let cli = Cli::parse();
    let converter = ArgConverter::new();
    let result = converter.convert(cli);
    println!("Running: gst-launch-1.0 {}\t", result.join(" "));
    let gst_launch = Exec::new("gst-launch-1.0".to_string(), result);
    gst_launch.exec();
}
