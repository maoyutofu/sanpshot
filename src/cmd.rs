use std::{
    io,
    process::{Child, Command, Stdio},
};

#[derive(Debug)]
pub struct CmdBuilder {
    pub options: Vec<Parameter>,
    pub cmd: String,
    pub stdin: Stdio,
    pub stdout: Stdio,
    pub stderr: Stdio,
}

#[derive(Debug)]
pub enum Parameter {
    Single(String),
    KeyValue(String, String),
}

impl Parameter {
    fn push_to(&self, command: &mut Command) {
        match &self {
            Parameter::Single(arg) => command.arg(arg),
            Parameter::KeyValue(key, value) => command.arg(key).arg(value),
        };
    }
}

#[derive(Debug)]
pub struct File {
    pub url: String,
    pub options: Vec<Parameter>,
}

impl File {
    pub fn new(url: String) -> Self {
        File {
            url,
            options: Vec::new(),
        }
    }

    pub fn option(mut self, option: Parameter) -> Self {
        self.options.push(option);
        self
    }

    pub fn add_options(mut self, options: &mut Vec<Parameter>) -> Self {
        self.options.append(options);
        self
    }
}

#[derive(Debug)]
pub struct Cmd {
    pub process: Child,
}

impl CmdBuilder {
    pub fn new(cmd: &str) -> Self {
        CmdBuilder {
            options: Vec::new(),
            cmd: cmd.into(),
            stdin: Stdio::null(),
            stdout: Stdio::null(),
            stderr: Stdio::null(),
        }
    }

    pub fn option(mut self, option: Parameter) -> Self {
        self.options.push(option);
        self
    }

    pub fn stdin(mut self, stdin: Stdio) -> Self {
        self.stdin = stdin;
        self
    }

    pub fn stdout(mut self, stdout: Stdio) -> Self {
        self.stdout = stdout;
        self
    }

    pub fn stderr(mut self, stderr: Stdio) -> Self {
        self.stderr = stderr;
        self
    }

    pub fn to_string(&self) -> String {
        let mut s = String::from(&self.cmd);
        for option in &self.options {
            s.push_str(" ");
            match option {
                Parameter::Single(op) => s.push_str(&op),
                Parameter::KeyValue(key, value) => {
                    s.push_str(&key);
                    s.push_str(" ");
                    s.push_str(&value);
                }
            }
        }
        s
    }

    pub fn to_command(self) -> Command {
        let mut command = Command::new(self.cmd);
        for option in self.options {
            option.push_to(&mut command);
        }

        command.stdin(self.stdin);
        command.stdout(self.stdout);
        command.stderr(self.stderr);
        command
    }

    pub fn run(self) -> io::Result<Cmd> {
        let mut command = self.to_command();
        let child = command.spawn()?;
        Ok(Cmd { process: child })
    }
}
