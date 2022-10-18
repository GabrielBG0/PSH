use super::github;
use std::{env, process::Command};

pub fn exec(url: String) {
    let root = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    github::clone(&url).unwrap();

    let split_url = url.split("/").collect::<Vec<&str>>();
    let mut repo_name = split_url[split_url.len() - 1];

    if repo_name.contains(".git") {
        repo_name = repo_name.split(".").collect::<Vec<&str>>()[0]
    }

    env::set_current_dir(format!("{}/{}", root, repo_name)).unwrap();

    if cfg!(target_os = "windows") {
        match Command::new("cmd").args(["/C", "yarn install"]).status() {
            Ok(_) => print!("nice"),
            Err(err) => print!("{:?}", err),
        }
    } else {
        match Command::new("sh").args(["-c", "yarn install"]).status() {
            Ok(_) => print!("nice"),
            Err(err) => print!("{:?}", err),
        }
    }
}
