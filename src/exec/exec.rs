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
}
