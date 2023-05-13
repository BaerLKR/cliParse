use colored::*;
pub mod arg;
fn main() {
    let cmd: Vec<arg::Argument> = arg::collect();
    for n in cmd {
        match n {
            arg::Argument::Test => println!("Test"),
            arg::Argument::Demo => demo(),
            arg::Argument::None | arg::Argument::Help => help(),
            arg::Argument::Path(x) => println!("{x}")
        }
    }
}
fn help() {
    println!("{}", "command <argument>".green());
    println!("{}", "Arguments: Help, Demo, Test, path (-p=<path>)".green());
    println!("{}", "When using this lib remember to adjust".red());
    println!("  {}", "1. The Enum".red());
    println!("  {}", "2. The match that checkes the provided Strings".red());
    println!("  {}", "3. The part in main.rs (fill the match with your functions)".red());
    println!("  {}", "4. The rules for the arguments".red());
    println!("  {}", "5. Maybe the evaluation fn".red());
}
fn demo() {
    for n in 0..100 {
        print!("{n:>2} ")
    }
    print!("\n");
}
