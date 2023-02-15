use std::os::unix::process::CommandExt;
use std::process::Command;

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
