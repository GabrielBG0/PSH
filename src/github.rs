use std::{error::Error, process::Command};

pub fn clone(url: &String) -> Result<(), Box<dyn Error>> {
    Command::new("git").args(["clone", url]).status()?;
    Ok(())
}
