use std::{env, fmt, fs, path::Path};

#[derive(Debug)]
pub struct CmdArgs {
    pub project: String,
    pub command: Command,
    pub function: Function,
}

impl CmdArgs {
    pub fn build(args: &[String]) -> Result<CmdArgs, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments provided");
        }
        let project = args[1].clone();
        let cmd_type = CommandTypes::validate_command_type(&args[2])?;
        let command = Command {
            command_type: cmd_type,
            command_str: args[2].clone(),
        };
        let function_type = FunctionTypes::validate_function_type(&args[3])?;
        let function = Function {
            func_type: function_type,
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

#[derive(Debug)]
pub struct Function {
    pub func_type: FunctionTypes,
    pub func_name: String,
    pub func_args: Vec<String>,
}

#[derive(Debug)]
pub struct Command {
    pub command_type: CommandTypes,
    pub command_str: String,
}

#[derive(Debug)]
pub struct TipCli {
    pub tip_dir: String,
    pub command: Command,
    pub function: Function,
}

#[derive(Debug, Clone, Copy)]
pub enum FunctionTypes {
    Project,
    Tip,
}

impl FunctionTypes {
    pub const fn to_str(&self) -> &'static str {
        match &self {
            FunctionTypes::Project => "@project",
            FunctionTypes::Tip => "@tip",
        }
    }
    pub fn validate_function_type(function: &str) -> Result<FunctionTypes, &'static str> {
        match function {
            "@project" => Ok(FunctionTypes::Project),
            "@tip" => Ok(FunctionTypes::Tip),
            _ => Err("Error: invalid function"),
        }
    }
}

// Adding the Display trait
impl fmt::Display for FunctionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CommandTypes {
    Create,
    Insert,
    Delete,
    Update,
}

impl CommandTypes {
    pub const fn to_str(&self) -> &'static str {
        match &self {
            CommandTypes::Create => "create",
            CommandTypes::Insert => "insert",
            CommandTypes::Delete => "delete",
            CommandTypes::Update => "update",
        }
    }
    pub fn validate_command_type(cmd: &str) -> Result<CommandTypes, &'static str> {
        match cmd {
            "create" => Ok(CommandTypes::Create),
            "insert" => Ok(CommandTypes::Insert),
            "delete" => Ok(CommandTypes::Delete),
            "update" => Ok(CommandTypes::Update),
            _ => Err("Error: invalid command"),
        }
    }
}

// Adding the Display trait
impl fmt::Display for CommandTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl TipCli {
    pub fn init(cmd_args: &CmdArgs) -> Result<TipCli, &'static str> {
        eprintln!("Initializing TipCli");
        let home_dir =
            env::var("HOME").map_err(|_| "Error: could not find the home directory: {err}")?;
        let tip_dir = format!("{home_dir}/.tip");
        if !Path::new(tip_dir.as_str()).is_dir() {
            fs::create_dir(&tip_dir).map_err(|_| "Error: tip dir couldn't be created")?;
        }

        let command = Command {
            command_type: cmd_args.command.command_type,
            command_str: cmd_args.command.command_str.clone(),
        };
        let function = Function {
            func_type: cmd_args.function.func_type,
            func_name: cmd_args.function.func_name.clone(),
            func_args: cmd_args.function.func_args.clone(),
        };
        Ok(TipCli {
            tip_dir,
            command,
            function,
        })
    }

    pub fn perform(&self) -> Result<(), &'static str> {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn function_type_validation_equal_works() {
        let function = "@project";
        assert!(
            match FunctionTypes::validate_function_type(function) {
                Ok(FunctionTypes::Project) => true,
                _ => false,
            },
            "Validate function types failed!"
        );
    }
    #[test]
    fn function_type_validation_not_equal_works() {
        let function = "@tip";
        assert!(
            match FunctionTypes::validate_function_type(function) {
                Ok(FunctionTypes::Project) => false,
                _ => true,
            },
            "Validate function types failed!"
        );
    }
    #[test]
    fn command_type_validation_equal_works() {
        let cmd = "insert";
        assert!(
            matches!(
                CommandTypes::validate_command_type(cmd),
                Ok(CommandTypes::Insert)
            ),
            "Validate command types failed!"
        );
    }
    #[test]
    fn command_type_validation_not_equal_works() {
        let cmd = "create";
        assert!(
            !matches!(
                CommandTypes::validate_command_type(cmd),
                Ok(CommandTypes::Insert) | Ok(CommandTypes::Delete) | Ok(CommandTypes::Update)
            ),
            "Validate command types failed!"
        );
    }
    #[test]
    fn perform_function_works() {
        let args = vec![
            String::from("tip"),
            String::from("insert"),
            String::from("@tip"),
        ];
        // Assuming these two will not error out.
        let cmd_args = CmdArgs::build(&args).unwrap();
        let tip_cli = TipCli::init(&cmd_args).unwrap();
        _ = tip_cli.perform();
    }
}
