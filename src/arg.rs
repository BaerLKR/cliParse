use std::collections::HashSet;
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Eq)]
pub enum Argument {
    // change what arguments are possible here!
    None,
    Help,
    Test,
    Demo,
    Path(String),
}
pub fn collect() -> Vec<Argument>{
    let arg: Vec<String> = std::env::args().collect();
    let arg = if arg.len() > 1 {
        Some(arg)
    } else {
        None
    };
    let re: Vec<Argument>;
    match arg {
        Some(v) => {
            re = eval(v);
        },
        None => {
            re = vec![Argument::None]
        }
    }
    return re;
}
fn eval(data: Vec<String>) -> Vec<Argument> {
    let mut re: Vec<Argument> = Vec::new();
    let mut c = 0;
    loop {
        c += 1;
        let n = &data[c];
        match n.as_str() {
            //chang how the arguments can be reached here!
            "help" | "-h" | "--help" => {
                re.push(Argument::Help);
            },
            "test" | "--test" | "-t" => {
                re.push(Argument::Test);
            },
            "demo" | "-d" => {
                re.push(Argument::Demo);
            },
            "-f" => {
                let x = &data[c + 1];
                let x = x.to_owned();
                re.push(Argument::Path(x));
                c += 1;
            }
            _ => {
                if n.chars().count() > 3 {
                    if  n.chars().nth(0).unwrap() == '-' &&
                        n.chars().nth(1).unwrap() == 'f' && 
                        n.chars().nth(2).unwrap() == '=' {
                            let mut n = n.to_owned();
                            n.remove(0);
                            n.remove(0);
                            n.remove(0);
                            re.push(Argument::Path(n));
                    } else {
                    }
                } else {
                    re.push(Argument::None);
                }
            }
        }
        if c >= (data.len() - 1) {
            break
        }
    }
    //add rules here
    //here I only count the "Help" argument when it is called once
    if re.contains(&Argument::Help) {
        re.clear();
        re.push(Argument::Help);
    }
    //or that it is not allowed to use the same argument 2 times
    match contains_duplicates(&re) {
        Some(v) => {
            re.remove(v);
        },
        None => {},
    };
    re
}
fn contains_duplicates(vec: &Vec<Argument>) -> Option<usize> {
    let mut seen = HashSet::new();
    for (i, x) in vec.iter().enumerate() {
        if !seen.insert(x) {
            return Some(i);
        }
    }
    None
}
