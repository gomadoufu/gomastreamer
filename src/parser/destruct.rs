use super::parse::{Cli, Command, InputFormat, InputType, OutputCommands, OutputFormat};

#[derive(Debug)]
pub struct Elements {
    pub itype: InputType,
    pub iformat: InputFormat,
    pub iwidth: i32,
    pub iheight: i32,
    pub oformat: OutputFormat,
    pub ohost: String,
    pub oport: i32,
    pub hardware: bool,
    pub dry_run: bool,
    pub show: bool,
}

impl Elements {
    pub fn destruct(cli: Cli) -> Elements {
        match cli.input {
            Command::Show => Elements {
                itype: InputType::Test,
                iformat: InputFormat::Yuy2,
                iwidth: 0,
                iheight: 0,
                oformat: OutputFormat::H264,
                ohost: "".to_string(),
                oport: 0,
                hardware: false,
                dry_run: false,
                show: true,
            },
            Command::Input(input) => {
                let itype = input.input_type;
                let iformat = input.format;
                match input.output {
                    OutputCommands::Output(output) => {
                        let iwidth = output.width;
                        let iheight = output.height;
                        let oformat = output.format;
                        let ohost = output.host;
                        let oport = output.port;
                        let hardware = output.hardware_encode;
                        let dry_run = output.dry_run;
                        let show = false;
                        Elements {
                            itype,
                            iformat,
                            iwidth,
                            iheight,
                            oformat,
                            ohost,
                            oport,
                            hardware,
                            dry_run,
                            show,
                        }
                    }
                }
            }
        }
    }
}
