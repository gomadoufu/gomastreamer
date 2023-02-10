mod exec;
mod parse;
use crate::exec::exec::Exec;
use crate::parse::{builder::ArgBuilder, parse::Cli};
use clap::Parser;
use parse::builder::MockCli;

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
    let mock = MockCli::new();
    let mut builder = ArgBuilder::new();
    let result = builder.build(mock);
    let gst_launch = Exec::new(
        "gst-launch-1.0".to_string(),
        vec![
            result[0].to_string(),
            "!".to_string(),
            "autovideosink".to_string(),
        ],
    );
    gst_launch.exec();
}
