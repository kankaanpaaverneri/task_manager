mod args;
mod files;
mod run;

fn main() {
    match args::parse_arguments() {
        Ok(config) => run::run(&config),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
