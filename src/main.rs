use std::{env, process};

struct Config {
    project: String,
    command: String,
    function: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments provided");
        }
        let project = args[1].clone();
        let command = args[2].clone();
        let function = args[3].clone();

        Ok(Config {
            project,
            command,
            function,
        })
    }
}

fn usage() {
    eprintln!("Usage: tip <project> [command] (function)");
    eprintln!("    -- project:");
    eprintln!("           project-name    The name of the project");
    eprintln!("    -- commands:");
    eprintln!("           create          to creae a new project");
    eprintln!("           insert          to add a new tip");
    eprintln!("           delete          to delete a new tip");
    eprintln!("           drop            to drop a project");
    eprintln!("    -- functions:");
    eprintln!("           @project        specify the name of the new project");
    eprintln!("           @tip            add a new tip!");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        usage();
        process::exit(1);
    });
    println!("so far so good.");
    println!("Config:");
    println!("project: {}", config.project);
    println!("command: {}", config.command);
    println!("function: {}", config.function);
}
