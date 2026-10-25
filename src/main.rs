use std::{env, fs, path::Path, process};

#[derive(Debug)]
struct Function {
    func_name: String,
    func_args: Vec<String>,
}

struct CmdArgs {
    project: String,
    command: String,
    function: Function,
}

impl CmdArgs {
    fn build(args: &[String]) -> Result<CmdArgs, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments provided");
        }
        let project = args[1].clone();
        let command = args[2].clone();
        let function = Function {
            func_name: args[3].clone(),
            func_args: args[4..].to_vec(),
        };

        Ok(CmdArgs {
            project,
            command,
            function,
        })
    }
}

// struct Tip {
//     home_dir: String,
//     project_dir: String,
//     command: String,
//     function: Function,
// }

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

fn find_tip_dir() {
    let home_dir = env::var("HOME").unwrap_or_else(|err| {
        eprintln!("Error: could not find the home directory: {err}");
        process::exit(1);
    });
    let tip_dir = format!("{home_dir}/.tip");

    if !Path::new(tip_dir.as_str()).is_dir() {
        fs::create_dir(tip_dir).expect("Directory couldn't be created");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = CmdArgs::build(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        usage();
        process::exit(1);
    });
    println!("so far so good.");
    println!("CmdArgs:");
    println!("    project: {}", config.project);
    println!("    command: {}", config.command);
    println!("    function: name {}", config.function.func_name);
    println!("              args {:?}", config.function.func_args);

    find_tip_dir();
}
