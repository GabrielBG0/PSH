use std::env;
mod github;
mod npm;
mod yarn;

pub use github::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 | 1 => {
            println!("Please pass a git url and a packege maneger like this: psh https://github.com/User/examp-npm-project-repo --npm");
            return;
        }
        2 => {
            println!(
                "A packege maneger flag is needed, use --yarn or --npm in the end of your command"
            );
            return;
        }
        3 => (),
        _ => {
            println!(
            "Too many arguments wore passed, try something n this format: psh https://github.com/User/examp-npm-project-repo --npm");
            return;
        }
    }

    let url = args[1].to_string();
    let flag = args[2].to_string();

    match flag.as_str() {
        "--yarn" => yarn::exec(url),
        "--npm" => npm::exec(url),
        _ => println!(
            "A valid packege maneger flag is needed, use --yarn or --npm in the end of your command"
        ),
    }
}
