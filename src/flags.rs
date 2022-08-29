use std::error::Error;

pub fn verify_flags(args: &Vec<String>) -> Result<bool, Box<dyn Error>> {
    match args.len() {
        0..=1 => panic!("No arguments provided"),
        2 => {
            println!("{:?}", args);
            if args[1].contains("https://") {
                panic!("No packege maneger provided");
            } else {
                panic!("No URL provided");
            }
        }
        3 => Ok(verify_tag(args)?),
        _ => panic!("Too many arguments wore passed"),
    }
}

fn verify_tag(args: &Vec<String>) -> Result<bool, Box<dyn Error>> {
    for arg in args.iter() {
        if arg.contains("-") {
            match arg.as_str() {
                "-npm" => return Ok(true),
                _ => panic!("{} is not valid packege maneger", arg),
            }
        }
    }
    Err
}
