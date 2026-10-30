use std::{env, fmt, fs, io::prelude::Write, path::Path};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FunctionTypes {
    Project,
    Tip,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandTypes {
    Create,
    Insert,
    Delete,
    Update,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub func_type: FunctionTypes,
    pub func_name: String,
    pub func_args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub command_type: CommandTypes,
    pub command_str: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CmdArgs {
    pub project: String,
    pub command: Command,
    pub function: Function,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TipCli {
    pub tip_dir: String,
    pub project_name: String,
    pub command: Command,
    pub function: Function,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tips {
    pub id: u32,
    pub title: String,
    pub critical: u8,
    pub done: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cursor {
    pub list: Option<Vec<Tips>>,
    pub ok: Result<bool, &'static str>,
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
        let project_name = cmd_args.project.clone();
        let command = cmd_args.command.clone();
        let function = cmd_args.function.clone();
        Ok(TipCli {
            tip_dir,
            project_name,
            command,
            function,
        })
    }

    pub fn perform(&self) -> Result<Cursor, &'static str> {
        match self.command.command_type {
            CommandTypes::Create => {
                if !self.function.func_args.is_empty() {
                    return Err("Invalid number of arguments provided: expected 0");
                }
                assert_eq!(self.function.func_type.to_str(), "@project");
                self.create_new_project()
            }
            _ => {
                unimplemented!();
            }
        }
    }

    fn create_new_project(&self) -> Result<Cursor, &'static str> {
        let new_project_dir_name = format!("{}/{}", self.tip_dir, self.project_name);
        fs::create_dir(&new_project_dir_name)
            .map_err(|_| "Error: couldn't create new project directory.")?;
        let currdir =
            env::current_dir().map_err(|_| "Error: couldn't get the current directory")?;
        let meta_data = format!(
            "@project({})\n@path({})\n@count(1)",
            self.project_name,
            currdir.display()
        );
        let mut meta_file = fs::File::create(format!("{}/000.tip", new_project_dir_name))
            .expect("Couldn't create metadata file"); // expect is temporary
        meta_file
            .write_all(meta_data.clone().as_bytes())
            .map_err(|_| "Couldn't write to metadata file")?;
        Ok(Cursor {
            list: None,
            ok: Ok(true),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn function_type_validation_equal_works() {
        let function = "@project";
        assert_eq!(
            FunctionTypes::validate_function_type(function),
            Ok(FunctionTypes::Project),
            "Validate function types failed!",
        );
    }
    #[test]
    fn function_type_validation_not_equal_works() {
        let function = "@tip";
        assert_ne!(
            FunctionTypes::validate_function_type(function),
            Ok(FunctionTypes::Project),
            "Validate function types failed!",
        );
    }
    #[test]
    fn command_type_validation_equal_works() {
        let cmd = "insert";
        assert_eq!(
            CommandTypes::validate_command_type(cmd),
            Ok(CommandTypes::Insert),
            "Validate command types failed!",
        );
    }
    #[test]
    fn command_type_validation_not_equal_works() {
        let cmd = "create";
        assert_eq!(
            CommandTypes::validate_command_type(cmd),
            Ok(CommandTypes::Create),
            "Validate command types failed!"
        );
    }
    #[test]
    fn perform_function_works() {
        let args = vec![
            String::from("current_project"),
            String::from("tip"),
            String::from("create"),
            String::from("@project"),
        ];
        // Assuming these two will not error out.
        fs::create_dir("temp").expect("temp dir not created");
        let cmd_args = CmdArgs::build(&args).expect("build failed");
        let mut tip_cli = TipCli::init(&cmd_args).expect("init failed");
        tip_cli.tip_dir = "./temp".to_string();
        let cursor_cmp = Cursor {
            list: None,
            ok: Ok(true),
        };
        let cursor_res = tip_cli.perform().expect("perform failed");
        assert_eq!(cursor_res, cursor_cmp);
        fs::remove_dir_all(tip_cli.tip_dir).expect("couldn't remove directory");
    }
}
