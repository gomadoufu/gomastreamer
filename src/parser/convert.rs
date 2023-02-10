use super::parse::{Cli, Format, Input, Resolution};
use std::{collections::HashMap, vec};

pub type MockCli = Cli;
impl MockCli {
    pub fn new() -> MockCli {
        MockCli {
            show: false,
            host: "localhost".to_string(),
            port: 8080,
            input: super::parse::Input::Test,
            resolution: super::parse::Resolution::Vga,
            format: super::parse::Format::Vp8,
            hardware_encode: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert() {
        let cli = MockCli::new();
        let mut converter = ArgConverter::new();
        println!("{:?}", converter.arg_map);
        let result = converter.convert(cli);
        println!("{:?}", result);
        assert_eq!(result[0], "videotestsrc");
        assert_eq!(result[1], "!");
        assert_eq!(result[2], "video/x-raw,width=640,height=480,framerate=30/1");
        assert_eq!(result[3], "!");
        assert_eq!(result[4], "videoconvert");
        assert_eq!(result[5], "!");
        assert_eq!(result[6], "autovideosink");
    }
}

#[derive(Debug)]
pub struct ArgConverter {
    arg_map: HashMap<String, String>,
}

impl ArgConverter {
    pub fn new() -> ArgConverter {
        let arg_map = make_args_vec()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        ArgConverter { arg_map }
    }
    pub fn convert(self, cli: MockCli) -> Vec<String> {
        let mut result = vec![];
        let input = match cli.input {
            Input::Test => self.arg_map.get("test").unwrap().to_string(),
            Input::Mipi => self.arg_map.get("mipi").unwrap().to_string(),
            Input::Usb => self.arg_map.get("usb").unwrap().to_string(),
        };
        result.push(input);
        result.push("!".to_string());
        let resolution = match cli.resolution {
            Resolution::Vga => self.arg_map.get("vga").unwrap().to_string(),
            Resolution::Sd => self.arg_map.get("sd").unwrap().to_string(),
            Resolution::Hd => self.arg_map.get("hd").unwrap().to_string(),
        };
        result.push(format!(
            "{}{}{}",
            "video/x-raw,", resolution, ",framerate=30/1"
        ));
        result.push("!".to_string());
        result.push("videoconvert".to_string());
        result.push("!".to_string());
        let format = match cli.format {
            Format::Vp8 if cli.hardware_encode => self.arg_map.get("vp8hard").unwrap().to_string(),
            Format::H264 if cli.hardware_encode => {
                self.arg_map.get("h264hard").unwrap().to_string()
            }
            Format::Vp8 => self.arg_map.get("vp8soft").unwrap().to_string(),
            Format::H264 => self.arg_map.get("h264soft").unwrap().to_string(),
        };
        result.push(format);
        result.push("!".to_string());
        let rtp = match cli.format {
            Format::Vp8 => self.arg_map.get("vp8rtp").unwrap().to_string(),
            Format::H264 => self.arg_map.get("h264rtp").unwrap().to_string(),
        };
        result.push(rtp);
        result.push("!".to_string());
        result.push("udpsink".to_string());
        result.push("".to_string());
        result.push(format!("host={}", cli.host));
        result.push("".to_string());
        result.push(format!("port={}", cli.port));

        result
    }
}

fn make_args_vec() -> Vec<(&'static str, &'static str)> {
    let result = vec![
        ("test", "videotestsrc"),
        ("mipi", "libcamerasrc"),
        ("usb", "-v v4l2src"),
        ("vga", "width=640,height=480"),
        ("sd", "width=800,height=600"),
        ("hd", "width=1280,height=720"),
        ("h264soft", "x264enc"),
        ("h264hard", "v4l2h264enc 'video/x-h254,level=(string)4'"),
        ("vp8rtp", "rtpvp8pay"),
        ("vp8soft", "vp8enc"),
    ];
    result
}
