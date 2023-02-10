use std::{collections::HashMap, vec};
pub struct MockCli {
    input: String,
}

impl MockCli {
    fn new() -> MockCli {
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
    fn new() -> ArgBuilder {
        let arg_map = [("test".to_string(), "videotestsrc".to_string())]
            .iter()
            .cloned()
            .collect();
        ArgBuilder { arg_map }
    }
    fn build(&self, cli: MockCli) -> Vec<String> {
        let mut result = vec![];
        result.push(self.arg_map.get(&cli.input).unwrap().to_string());
        result
    }
}
