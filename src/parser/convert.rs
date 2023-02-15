use super::parse::{Cli, Format, Input, Resolution};
use std::{collections::HashMap, vec};

#[derive(Debug)]
pub struct ArgConverter {
    arg_map: HashMap<String, String>,
}

impl ArgConverter {
    pub fn new() -> ArgConverter {
        let arg_map = map_args()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        ArgConverter { arg_map }
    }
    pub fn convert(self, cli: Cli) -> Vec<String> {
        let mut result = vec![];
        result.push(self.match_input(&cli));
        result.push("!".to_string());
        result.push(format!(
            "{}{}{}",
            "video/x-raw,",
            self.match_resolution(&cli),
            ",framerate=30/1"
        ));
        result.push("!".to_string());
        result.push("videoconvert".to_string());
        result.push("!".to_string());
        result.push(self.match_format(&cli));
        result.push("!".to_string());
        result.push(self.select_rtp(&cli));
        result.push("!".to_string());
        result.push("udpsink".to_string());
        result.push("".to_string());
        result.push(format!("host={}", cli.host));
        result.push("".to_string());
        result.push(format!("port={}", cli.port));

        result
    }
    fn match_input(&self, cli: &Cli) -> String {
        match cli.input {
            Input::Test => self.arg_map.get("test").unwrap().to_string(),
            Input::Mipi => self.arg_map.get("mipi").unwrap().to_string(),
            Input::Usb => self.arg_map.get("usb").unwrap().to_string(),
        }
    }

    fn match_resolution(&self, cli: &Cli) -> String {
        match cli.resolution {
            Resolution::Vga => self.arg_map.get("vga").unwrap().to_string(),
            Resolution::Sd => self.arg_map.get("sd").unwrap().to_string(),
            Resolution::Hd => self.arg_map.get("hd").unwrap().to_string(),
        }
    }

    fn match_format(&self, cli: &Cli) -> String {
        match cli.format {
            Format::Vp8 if cli.hardware_encode => self.arg_map.get("vp8hard").unwrap().to_string(),
            Format::H264 if cli.hardware_encode => {
                self.arg_map.get("h264hard").unwrap().to_string()
            }
            Format::Vp8 => self.arg_map.get("vp8soft").unwrap().to_string(),
            Format::H264 => self.arg_map.get("h264soft").unwrap().to_string(),
        }
    }

    fn select_rtp(&self, cli: &Cli) -> String {
        match cli.format {
            Format::Vp8 => self.arg_map.get("vp8rtp").unwrap().to_string(),
            Format::H264 => self.arg_map.get("h264rtp").unwrap().to_string(),
        }
    }
}

fn map_args() -> Vec<(&'static str, &'static str)> {
    let result = vec![
        ("test", "videotestsrc"),
        ("mipi", "libcamerasrc"),
        ("usb", "-v v4l2src"),
        ("vga", "width=640,height=480"),
        ("sd", "width=720,height=480"),
        ("hd", "width=1280,height=720"),
        ("h264soft", "x264enc"),
        ("h264hard", "v4l2h264enc ! 'video/x-h264,level=(string)4'"),
        ("vp8rtp", "rtpvp8pay"),
        ("h264rtp", "rtph264pay"),
        ("vp8soft", "vp8enc"),
        ("vp8hard", "omxvp8enc"),
    ];
    result
}
