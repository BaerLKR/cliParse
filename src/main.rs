use colored::*;
pub mod arg;
fn main() {
    let cmd: Vec<arg::Argument> = arg::collect();
    // for n in cmd {
    //     for i in 0.. {
    //         if n == cmd[i] {
    //             cmd.remove(i);
    //         }
    //     }
    //  }
    for n in cmd {
        match n {
            arg::Argument::Help => println!("{}", "Help".yellow()),
            arg::Argument::Test => println!("Test"),
            _ => {}
        }
    }
}
