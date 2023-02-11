use std::os::unix::process::CommandExt;
use std::process::Command;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        let mut cmd = Command::new("echo");
        cmd.arg("hello world");
        cmd.exec();
    }

    #[test]
    fn test_gstinspect() {
        let mut cmd = Command::new("gst-inspect-1.0");
        cmd.arg("videotestsrc");
        cmd.exec();
    }

    #[test]
    fn test_gstlaunch() {
        let mut cmd = Command::new("gst-launch-1.0");
        cmd.arg("videotestsrc");
        cmd.arg("!");
        cmd.arg("autovideosink");
        cmd.exec();
    }

    #[test]
    fn test_gstlaunch_exec() {
        let gst_launch = Exec::new(
            "gst-launch-1.0".to_string(),
            vec![
                "videotestsrc".to_string(),
                "!".to_string(),
                "autovideosink".to_string(),
            ],
        );
        gst_launch.exec();
    }

    #[test]
    fn test_exec_exec() {
        let exec = Exec::new("echo".to_string(), vec!["hello world".to_string()]);
        exec.exec();
    }
}

#[derive(Debug)]
pub struct Exec {
    pub command: String,
    pub args: Vec<String>,
}

impl Exec {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }

    pub fn exec(&self) {
        let mut cmd = Command::new(&self.command);
        self.args.iter().for_each(|arg| {
            cmd.arg(arg);
        });
        cmd.exec();
    }
}
