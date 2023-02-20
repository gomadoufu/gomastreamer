use crate::parser::{
    destruct::Elements,
    parse::{InputType, OutputFormat},
};

use std::{collections::HashMap, vec};

#[derive(Debug)]
pub struct Builder {
    arguments: Elements,
    arg_map: HashMap<String, String>,
}

impl Builder {
    pub fn new(arguments: Elements) -> Builder {
        let arg_map = map_args()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        Builder { arguments, arg_map }
    }
    pub fn build(&self) -> Vec<String> {
        let mut result = vec![];
        result.push(self.match_input_type());
        result.push("!".to_string());
        // カメラから、指定された解像度の映像を取得しておく。後で変換しない。
        result.push(format!(
            "{},{},{}",
            self.arguments.iformat,
            format_args!("width={}", self.arguments.iwidth),
            format_args!("height={}", self.arguments.iheight),
        ));
        result.push("!".to_string());
        result.push("videoconvert".to_string());
        result.push("!".to_string());
        result.push(self.match_oformat());
        result.push("!".to_string());
        result.push(self.match_rtp());
        result.push("!".to_string());
        result.push("udpsink".to_string());
        result.push(format!("host={}", self.arguments.ohost));
        result.push(format!("port={}", self.arguments.oport));

        result
    }

    fn match_input_type(&self) -> String {
        match self.arguments.itype {
            InputType::Test => self.arg_map.get("test").unwrap().to_string(),
            InputType::Mipi => self.arg_map.get("mipi").unwrap().to_string(),
            InputType::Usb => self.arg_map.get("usb").unwrap().to_string(),
        }
    }

    fn match_oformat(&self) -> String {
        match &self.arguments.oformat {
            OutputFormat::H264 if self.arguments.hardware => {
                self.arg_map.get("h264hard").unwrap().to_string()
            }
            OutputFormat::H264 => self.arg_map.get("h264soft").unwrap().to_string(),
            OutputFormat::Vp8 if self.arguments.hardware => {
                self.arg_map.get("vp8hard").unwrap().to_string()
            }
            OutputFormat::Vp8 => self.arg_map.get("vp8soft").unwrap().to_string(),
        }
    }

    fn match_rtp(&self) -> String {
        match self.arguments.oformat {
            OutputFormat::H264 => self.arg_map.get("h264rtp").unwrap().to_string(),
            OutputFormat::Vp8 => self.arg_map.get("vp8rtp").unwrap().to_string(),
        }
    }
}

fn map_args() -> Vec<(&'static str, &'static str)> {
    let result = vec![
        ("test", "videotestsrc"),
        ("mipi", "libcamerasrc"),
        ("usb", "-v v4l2src"),
        ("h264soft", "x264enc"),
        ("h264hard", "v4l2h264enc ! 'video/x-h264,level=(string)4'"),
        ("vp8soft", "vp8enc"),
        ("vp8hard", "omxvp8enc"),
        ("vp8rtp", "rtpvp8pay"),
        ("h264rtp", "rtph264pay"),
    ];
    result
}
