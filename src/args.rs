use std::env;

const MIN_ARGUMENTS: usize = 2;
const MAX_ARGUMENTS: usize = 3;

pub struct Config {
    pub first: String,
    pub second: String,
}

pub fn parse_arguments<'a>() -> Result<Config, &'a str> {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < MIN_ARGUMENTS {
        return Err("Not enough arguments.");
    }

    if arguments.len() > MAX_ARGUMENTS {
        return Err("Too many arguments.");
    }

    let config = Config {
        first: arguments[1].clone(),
        second: String::from(""),
    };

    if arguments.len() == 3 {
        let config = Config {
            first: arguments[1].clone(),
            second: arguments[2].clone(),
        };
        return Ok(config);
    }

    return Ok(config);
}
