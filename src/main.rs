use std::{env, process};

use tip::{CmdArgs, TipCli};

fn usage() {
    eprintln!("Usage: tip <project> [command] (function)");
    eprintln!("    -- project:");
    eprintln!("           project-name    The name of the project");
    eprintln!("    -- commands:");
    eprintln!("           create          to creae a new project");
    eprintln!("           insert          to add a new tip");
    eprintln!("           delete          to delete a tip");
    eprintln!("           drop            to drop a project");
    eprintln!("    -- functions:");
    eprintln!("           @project        specify the name of the new project");
    eprintln!("           @tip            add a new tip!");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd_args = CmdArgs::build(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}\n");
        usage();
        process::exit(1);
    });
    println!("so far so good.");
    println!("CmdArgs:");
    println!("    project: {}", cmd_args.project);
    println!("    command: {}", cmd_args.command.command_str);
    println!("    function: name {}", cmd_args.function.func_name);
    println!("              args {:?}", cmd_args.function.func_args);
    let tip_cli = TipCli::init(&cmd_args).expect("ERROR HAPPENED");
    println!("{:?}", tip_cli);
    _ = tip_cli.perform().expect("perform doesn't work");
}
