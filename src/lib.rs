use std::{env, fmt, fs, io::prelude::Write, path::Path};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FunctionTypes {
    Project,
    Tip,
    Where,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandTypes {
    Create,
    Delete,
    Update,
    Show,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub func_type: FunctionTypes,
    pub func_args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub command_type: CommandTypes,
    pub command_func: Function,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TipCli {
    pub tip_dir: String,
    pub project_name: String,
    pub command: Command,
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

impl FunctionTypes {
    pub const fn to_str(&self) -> &'static str {
        match &self {
            FunctionTypes::Project => "@project",
            FunctionTypes::Tip => "@tip",
            FunctionTypes::Where => "@where",
        }
    }
    pub fn new(function: &str) -> Result<FunctionTypes, String> {
        match function {
            "@project" => Ok(FunctionTypes::Project),
            "@tip" => Ok(FunctionTypes::Tip),
            "@where" => Ok(FunctionTypes::Where),
            _ => Err("Error: invalid function".to_string()),
        }
    }
}

// Adding the Display trait
impl fmt::Display for FunctionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl Function {
    pub fn new(func: &str, args: Vec<String>) -> Result<Function, String> {
        let fn_type = FunctionTypes::new(func).expect("Function type validation failed");
        match fn_type {
            FunctionTypes::Project => {
                // the @project function accepts 0 arguments
                if !args.is_empty() {
                    return Err("Wrong number of arguments provided: expected none.".to_string());
                }
                Ok(Function {
                    func_args: args.clone(),
                    func_type: fn_type,
                })
            }
            FunctionTypes::Tip => unimplemented!(),
            _ => Err("Not implimented".to_string()),
        }
    }
}

impl CommandTypes {
    pub const fn to_str(&self) -> &'static str {
        match &self {
            CommandTypes::Create => "create",
            CommandTypes::Delete => "delete",
            CommandTypes::Update => "update",
            CommandTypes::Show => "show",
        }
    }
    pub fn new(cmd: &str) -> Result<CommandTypes, String> {
        match cmd {
            "create" => Ok(CommandTypes::Create),
            "delete" => Ok(CommandTypes::Delete),
            "update" => Ok(CommandTypes::Update),
            "show" => Ok(CommandTypes::Show),
            _ => Err("Error: invalid command".to_string()),
        }
    }
}

// Adding the Display trait
impl fmt::Display for CommandTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl Command {
    pub fn new(cmd_str: &str, cmd_func: Function) -> Result<Command, String> {
        let cmd_type = CommandTypes::new(cmd_str)?;
        Ok(Command {
            command_type: cmd_type,
            command_func: cmd_func.clone(),
        })
    }
}

impl TipCli {
    pub fn init(cmd_args: &[String]) -> Result<TipCli, String> {
        eprintln!("Initializing TipCli");
        let home_dir =
            env::var("HOME").map_err(|_| "Error: could not find the home directory: {err}")?;
        let tip_dir = format!("{home_dir}/.tip");
        if !Path::new(tip_dir.as_str()).is_dir() {
            fs::create_dir(&tip_dir).map_err(|_| "Error: tip dir couldn't be created")?;
        }
        let project_name = cmd_args[1].clone();
        let function = Function::new(cmd_args[3].as_str(), cmd_args[4..].to_vec())?;
        let command = Command::new(cmd_args[2].as_str(), function)?;
        Ok(TipCli {
            tip_dir,
            project_name,
            command,
        })
    }

    pub fn perform(&self) -> Result<Cursor, String> {
        match self.command.command_type {
            CommandTypes::Create => match self.command.command_func.func_type {
                FunctionTypes::Project => {
                    assert_eq!(self.command.command_func.func_type.to_str(), "@project");
                    self.create_new_project()
                }
                other => Err(format!(
                    "Error: invalid function for this command: {}",
                    other.to_str()
                )),
            },
            _ => {
                unimplemented!();
            }
        }
    }

    fn create_new_project(&self) -> Result<Cursor, String> {
        let new_project_dir_name = format!("{}/{}", self.tip_dir, self.project_name);
        fs::create_dir(&new_project_dir_name)
            .map_err(|_| "Error: couldn't create new project directory.")?;
        let currdir =
            env::current_dir().map_err(|_| "Error: couldn't get the current directory")?;
        let meta_data = format!(
            "@project({})\n@path({})\n@count(1)\n",
            self.project_name,
            currdir.display()
        );
        let mut meta_file = fs::File::create(format!("{}/000.tip", new_project_dir_name))
            .map_err(|_| "Error: couldn't create metadata file")?;
        meta_file
            .write_all(meta_data.clone().as_bytes())
            .map_err(|_| "Error: couldn't write to metadata file")?;
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
            FunctionTypes::new(function),
            Ok(FunctionTypes::Project),
            "Validate function types failed!",
        );
    }
    #[test]
    fn function_type_validation_not_equal_works() {
        let function = "@tip";
        assert_ne!(
            FunctionTypes::new(function),
            Ok(FunctionTypes::Project),
            "Validate function types failed!",
        );
    }
    #[test]
    fn command_type_validation_equal_works() {
        let cmd = "delete";
        assert_eq!(
            CommandTypes::new(cmd),
            Ok(CommandTypes::Delete),
            "Validate command types failed!",
        );
    }
    #[test]
    fn command_type_validation_not_equal_works() {
        let cmd = "delete";
        assert_ne!(
            CommandTypes::new(cmd),
            Ok(CommandTypes::Create),
            "Validate command types failed!"
        );
    }
    #[test]
    fn function_validation_equal_works() {
        let func_res = Function::new("@project", vec![]);
        let func_cmp = Function {
            func_type: FunctionTypes::Project,
            func_args: vec![],
        };
        assert_eq!(func_res, Ok(func_cmp));
    }
    #[test]
    fn command_validation_equal_works() {
        let _func_cmp = Function {
            func_type: FunctionTypes::Project,
            func_args: vec![],
        };
        let cmd_res = Command::new("create", _func_cmp.clone());
        let cmd_cmp = Command {
            command_type: CommandTypes::Create,
            command_func: _func_cmp,
        };
        assert_eq!(cmd_res, Ok(cmd_cmp));
    }
    #[test]
    fn perform_function_works() {
        let args = vec![
            String::from("current_project"),
            String::from("tip"),
            String::from("create"),
            String::from("@project"),
        ];
        // Assuming these will not error out.
        fs::create_dir("temp").expect("temp dir not created");
        let mut tip_cli = TipCli::init(&args).expect("init failed");
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
