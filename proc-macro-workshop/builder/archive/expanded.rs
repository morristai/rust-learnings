use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use derive_builder::Builder;
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}
pub struct CommandBuilder {
    executable: std::option::Option<String>,
    args: std::option::Option<Vec<String>>,
    env: std::option::Option<Vec<String>>,
    current_dir: std::option::Option<Option<String>>,
}
impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: std::option::Option::None,
            args: std::option::Option::None,
            env: std::option::Option::None,
            current_dir: std::option::Option::None,
        }
    }
}
impl CommandBuilder {
    fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = std::option::Option::Some(executable);
        self
    }
    fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = std::option::Option::Some(args);
        self
    }
    fn env(&mut self, env: Vec<String>) -> &mut Self {
        self.env = std::option::Option::Some(env);
        self
    }
    fn current_dir(&mut self, current_dir: Option<String>) -> &mut Self {
        self.current_dir = std::option::Option::Some(current_dir);
        self
    }
    pub fn build(
        &mut self,
    ) -> std::result::Result<Command, std::boxed::Box<dyn std::error::Error>> {
        if self.executable.is_none() {
            let err = {
                let res = ::alloc::fmt::format(
                    format_args!("{0} field missing", "executable"),
                );
                res
            };
            return std::result::Result::Err(err.into());
        }
        if self.args.is_none() {
            let err = {
                let res = ::alloc::fmt::format(
                    format_args!("{0} field missing", "args"),
                );
                res
            };
            return std::result::Result::Err(err.into());
        }
        if self.env.is_none() {
            let err = {
                let res = ::alloc::fmt::format(format_args!("{0} field missing", "env"));
                res
            };
            return std::result::Result::Err(err.into());
        }
        if self.current_dir.is_none() {
            let err = {
                let res = ::alloc::fmt::format(
                    format_args!("{0} field missing", "current_dir"),
                );
                res
            };
            return std::result::Result::Err(err.into());
        }
        let ret = Command {
            executable: self.executable.clone().unwrap(),
            args: self.args.clone().unwrap(),
            env: self.env.clone().unwrap(),
            current_dir: self.current_dir.clone().unwrap(),
        };
        std::result::Result::Ok(ret)
    }
}
fn main() {
    let builder = Command::builder();
    let _ = builder;
}