use std::env;
mod github;
mod npm;
mod yarn;

pub use github::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = args[1].to_string();
    let flag = args[2].to_string();

    match flag.as_str() {
        "--yarn" => yarn::exec(url),
        "--npm" => npm::exec(url),
        _ => panic!(
            "A packege maneger flag is needed, use --yarn or --npm in the end of yout command"
        ),
    }
}
