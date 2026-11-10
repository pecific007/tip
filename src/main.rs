use std::{env, process};

use tip::TipCli;

fn usage() {
    eprintln!("Usage: tip <project> [command] (function)");
    eprintln!("    -- project:");
    eprintln!("          <project>        The name of the project");
    eprintln!("    -- commands:");
    eprintln!("           create          to create project|tip"); // sort of implemented
    eprintln!("           update          to update project|tip"); // not implemented
    eprintln!("           delete          to delete project|tip"); // not implemented
    eprintln!("           show            to show   project|tip"); // not implemented
    eprintln!("    -- functions:");
    eprintln!("           @project        create new project"); // sort of implemented
    eprintln!("           @tip            create new tip"); // sort of implemented
    eprintln!("           @where          specify a condition"); // not implemented
    eprintln!("Not implemented: use `help command|function <name>` to learn more about them."); // not implemented
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
    println!("Args:");
    println!("    project: {}", args[1]);
    println!("    command: {}", args[2]);
    println!("    function: name {}", args[3]);
    println!("              args {:?}", args[4..].to_vec());
    let tip_cli = TipCli::init(&args).unwrap_or_else(|e| {
        eprintln!("Error: TipCli::init failed: {e}");
        process::exit(1);
    });
    println!("{:?}\n", tip_cli);
    let cursor = tip_cli.perform();
    cursor.ok.as_ref().unwrap_or_else(|e| {
        eprintln!("Error: TipCli::perform failed: {e}");
        usage();
        process::exit(1);
    });
    println!("\n//---- Cursor ----//\n");
    cursor.print_cursor_list();
}
