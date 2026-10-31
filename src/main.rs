use std::{env, process};
use tip::TipCli;

fn usage() {
    eprintln!("Usage: tip <project> [command] (function)");
    eprintln!("    -- project:");
    eprintln!("          <project>        The name of the project");
    eprintln!("    -- commands:");
    eprintln!("           create          to create project|tip"); // sort of implimented
    eprintln!("           update          to update project|tip"); // not implimented
    eprintln!("           delete          to delete project|tip"); // not implimented
    eprintln!("           show            to show   project|tip"); // not implimented
    eprintln!("    -- functions:");
    eprintln!("           @project        create new project"); // sort of implimented
    eprintln!("           @tip            create new tip"); // not implimented
    eprintln!("           @where          specify a condition"); // not implimented
    eprintln!("use `help command|function <name>` to learn more about them."); // not implimented
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1..].to_vec().contains(&"help".to_string()) {
        if args.len() > 3 {
            eprintln!("Make sure to quote the arguments like title");
        }
        usage();
        process::exit(0);
    }
    if args.len() < 4 {
        eprintln!("Error: Not enough arguments provided");
        usage();
        process::exit(1);
    }
    println!("so far so good.");
    println!("CmdArgs:");
    println!("    project: {}", args[1]);
    println!("    command: {}", args[2]);
    println!("    function: name {}", args[3]);
    println!("              args {:?}", args[4..].to_vec());
    let tip_cli = TipCli::init(&args).unwrap_or_else(|e| {
        eprintln!("Error: TipCli::init failed: {e}");
        process::exit(1);
    });
    println!("{:?}", tip_cli);
    _ = tip_cli.perform().unwrap_or_else(|e| {
        eprintln!("Error: TipCli::perform failed: {e}");
        usage();
        process::exit(1);
    });
}
