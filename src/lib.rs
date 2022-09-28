pub mod kanshi;
pub mod executer;
pub mod log;

use std::process::Command;

pub fn script(scrpt: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(scrpt)
        .output()
        .expect("failed to execute process")
        .stdout;
}
