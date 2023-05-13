use colored::*;
pub mod arg;
fn main() {
    let cmd: Vec<arg::Argument> = arg::collect();
    for n in cmd {
        match n {
            arg::Argument::Test => println!("Test"),
            arg::Argument::Demo => demo(),
            arg::Argument::None | arg::Argument::Help => help(),
        }
    }
}
fn help() {
    println!("{}", "command <argument>".green());
    println!("{}", "Arguments: Help, Demo, Test".green());
}
fn demo() {
    for n in 0..100 {
        print!("{n:>2} ")
    }
    print!("\n");
}
