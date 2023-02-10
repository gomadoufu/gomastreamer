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
}
