use std::env;
mod github;
mod npm;

pub use github::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = args[1].to_string();
    npm::exec(url);
}
