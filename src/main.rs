use colored::*;
pub mod arg;
fn main() {
    let cmd: Vec<arg::Argument> = arg::collect();
    for n in cmd {
        match n {
            arg::Argument::Help => println!("{}", "Help".yellow()),
            arg::Argument::Test => println!("Test"),
            _ => {}
        }
    }
}
