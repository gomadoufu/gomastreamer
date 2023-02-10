use std::{collections::HashMap, vec};
pub struct MockCli {
    input: String,
}

impl MockCli {
    pub fn new() -> MockCli {
        MockCli {
            input: "test".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mock() {
        let cli = MockCli::new();
        assert_eq!(cli.input, "test");
    }
    #[test]
    fn test_build() {
        let cli = MockCli::new();
        let mut builder = ArgBuilder::new();
        let result = builder.build(cli);
        assert_eq!(result[0], "videotestsrc");
    }
}

pub struct ArgBuilder {
    arg_map: HashMap<String, String>,
}

impl ArgBuilder {
    pub fn new() -> ArgBuilder {
        let arg_map = [("test".to_string(), "videotestsrc".to_string())]
            .iter()
            .cloned()
            .collect();
        ArgBuilder { arg_map }
    }
    pub fn build(&self, cli: MockCli) -> Vec<String> {
        let mut result = vec![];
        result.push(self.arg_map.get(&cli.input).unwrap().to_string());
        result
    }
}

fn create_mapping_vec() -> Vec<(&'static str, &'static str)> {
    let mut result = vec![
        ("test", "videotestsrc"),
        ("mipi", "libcamerasrc"),
        ("usb", "-v v4l2src"),
        ("vga", "640x480"),
        ("sd", "1280x720"),
        ("hd", "1920x1080"),
        ("h264rtp", "rtph264pay"),
        ("h264soft", "x264enc"),
        ("h264hard", "v4l2h264enc 'video/x-h254,level=(string)4'"),
        ("vp8rtp", "rtpvp8pay"),
        ("vp8soft", "vp8enc"),
    ];
    result
}
