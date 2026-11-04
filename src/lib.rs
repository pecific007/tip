// TODO: separate it into different crates/modules
use std::{env, fmt, fs, io::prelude::*, path::Path};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseFile {
    Metadata,
    TipFile,
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
pub struct TipMeta {
    pub project: String,
    pub path: String,
    pub count: u32,
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
    pub ok: Result<(), &'static str>,
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
        let fn_type = FunctionTypes::new(func)?;
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
            FunctionTypes::Tip => {
                if args.len() != 2 {
                    return Err("Wrong number of arguments provided: expected 2".to_string());
                }
                Ok(Function {
                    func_args: args.clone(),
                    func_type: fn_type,
                })
            }
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
    pub fn new(
        cmd_str: &str,
        cmd_func_name: &str,
        cmd_func_args: Vec<String>,
    ) -> Result<Command, String> {
        let function = Function::new(cmd_func_name, cmd_func_args)?;
        let cmd_type = CommandTypes::new(cmd_str)?;
        Ok(Command {
            command_type: cmd_type,
            command_func: function,
        })
    }
}

impl TipCli {
    pub fn init(cmd_args: &[String]) -> Result<TipCli, String> {
        let home_dir =
            env::var("HOME").map_err(|_| "Error: could not find the home directory: {err}")?;
        let tip_dir = format!("{home_dir}/.tip");
        if !Path::new(tip_dir.as_str()).is_dir() {
            fs::create_dir(&tip_dir).map_err(|_| "Error: tip dir couldn't be created")?;
        }
        let project_name = cmd_args[1].clone();
        let command = Command::new(
            cmd_args[2].as_str(),
            cmd_args[3].as_str(),
            cmd_args[4..].to_vec(),
        )?;
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
                FunctionTypes::Tip => {
                    assert_eq!(self.command.command_func.func_type.to_str(), "@tip");
                    self.create_new_tip()
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
        if let Ok(f) = fs::create_dir(&new_project_dir_name) {
            f
        };
        let currdir = env::current_dir()
            .map_err(|e| format!("Error: couldn't get the current directory: {e}"))?;
        let meta_data = format!(
            "@project({})\n@path({})\n@count(1)\n",
            self.project_name,
            currdir.display()
        );
        let mut meta_file = fs::File::create(format!("{}/000.tip", new_project_dir_name))
            .map_err(|e| format!("Error: couldn't create metadata file: {e}"))?;
        meta_file
            .write_all(meta_data.clone().as_bytes())
            .map_err(|e| format!("Error: couldn't write to metadata file: {e}"))?;
        Ok(Cursor {
            list: None,
            ok: Ok(()),
        })
    }

    fn create_new_tip(&self) -> Result<Cursor, String> {
        let meta_file =
            fs::read_to_string(format!("{}/{}/000.tip", self.tip_dir, self.project_name).as_str())
                .map_err(|e| format!("Error opening metadaa file: {e}"))?;
        let mut meta_parsed = parse_meta_file(&meta_file)?;
        let new_tip_file_path = format!(
            "{}/{}/{:03}.tip",
            self.tip_dir, self.project_name, meta_parsed.count
        );
        let mut new_tip_file =
            fs::File::create(new_tip_file_path).map_err(|_| "Error: couldn't create tip file")?;
        let func_args = self.command.command_func.func_args.clone();
        let new_file_content = format!(
            "@title({})\n@critical({})\n@done({})\n",
            func_args[1], func_args[0], false
        );
        new_tip_file
            .write_all(new_file_content.as_bytes())
            .map_err(|e| format!("Couldn't write to new tip file: {e}"))?;
        meta_parsed.update_meta_for_new_tip(self)?;
        Ok(Cursor {
            list: None,
            ok: Ok(()),
        })
    }
}

impl TipMeta {
    pub fn update_meta_for_new_tip(&mut self, tip_cli: &TipCli) -> Result<(), String> {
        let path = format!("{}/{}/000.tip", tip_cli.tip_dir, tip_cli.project_name);
        let path_tmp = format!("{path}.tmp");
        self.count += 1;
        let meta_data = format!(
            "@project({})\n@path({})\n@count({})\n",
            self.project, self.path, self.count,
        );
        fs::write(&path_tmp, &meta_data)
            .map_err(|e| format!("Error: couldn't open tip meta file: {e}"))?;
        fs::rename(&path_tmp, &path)
            .map_err(|e| format!("Error: couldn't open tip meta file: {e}"))?;
        Ok(())
    }
}

fn store_data(var: &mut String, line: &str) {
    let mut buffer = String::new();
    let mut in_paren = false;

    for ch in line.chars() {
        match ch {
            '(' => {
                in_paren = true;
                buffer.clear();
            }
            ')' if in_paren => {
                *var = std::mem::take(&mut buffer);
                in_paren = false;
            }
            c if in_paren => buffer.push(c),
            _ => {}
        }
    }
}

fn parse_meta_file(content: &str) -> Result<TipMeta, String> {
    let mut project_name: String = String::new();
    let mut project_path: String = String::new();
    let mut project_count: String = String::new();
    for line in content.lines() {
        if line.starts_with("@project") {
            store_data(&mut project_name, line);
        } else if line.starts_with("@path") {
            store_data(&mut project_path, line);
        } else if line.starts_with("@count") {
            store_data(&mut project_count, line);
        }
    }
    if project_name.is_empty() || project_path.is_empty() || project_count.is_empty() {
        return Err(String::from(
            "Error: not enough information in the metadata file.",
        ));
    }
    let id_count = project_count
        .trim()
        .parse()
        .map_err(|e| format!("Error: {e}"))?;
    Ok(TipMeta {
        project: project_name.clone(),
        path: project_path.clone(),
        count: id_count,
    })
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
        let func_got = Function::new("@project", vec![]);
        let func_want = Function {
            func_type: FunctionTypes::Project,
            func_args: vec![],
        };
        assert_eq!(func_got, Ok(func_want));
    }
    #[test]
    fn command_validation_equal_works() {
        let _func_want = Function::new("@project", vec![])
            .expect("failed to create a function to make the command");
        let cmd_got = Command::new("create", "@project", vec![]);
        let cmd_want = Command {
            command_type: CommandTypes::Create,
            command_func: _func_want,
        };
        assert_eq!(cmd_got, Ok(cmd_want));
    }
    #[test]
    fn tip_init_works() {
        let args = vec![
            String::from("current_project"),
            String::from("tip"),
            String::from("create"),
            String::from("@project"),
        ];
        let tip_cli = TipCli::init(&args).expect("init failed");
        let command = Command::new("create", "@project", vec![]).expect("command new failed");
        let tip_dir = "/home/prashant/.tip".to_string();
        let project_name = args[1].clone();
        let tip_want = TipCli {
            tip_dir,
            project_name,
            command,
        };
        assert_eq!(tip_cli, tip_want);
    }
    #[test]
    fn perform_function_create_works() {
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
        let cursor_want = Cursor {
            list: None,
            ok: Ok(()),
        };
        let cursor_got = tip_cli.perform().expect("perform failed");
        assert_eq!(cursor_got, cursor_want);
        fs::remove_dir_all(tip_cli.tip_dir)
            .map_err(|e| format!("couldn't remove directory: {e}"))
            .unwrap();
    }
    #[test]
    fn test_store_data_works() {
        let line = "(FirstTest)";
        let mut var = String::new();
        store_data(&mut var, line);
        assert_eq!(var, "FirstTest");
    }
    #[test]
    fn test_store_data_not_works() {
        let line = "(SecondTest)";
        let mut var = String::new();
        store_data(&mut var, line);
        assert_ne!(var, "FirstTest");
    }
    #[test]
    fn test_parse_meta_file_works() {
        let content = "@project(test)\n@path(/home/user/.tip/test/)\n@count(1)\n";
        let tip_meta_got = parse_meta_file(content).expect("Couldn't parse meta file");
        let tip_meta_want = TipMeta {
            project: "test".to_string(),
            path: "/home/user/.tip/test/".to_string(),
            count: 1,
        };
        assert_eq!(tip_meta_got, tip_meta_want);
    }
    #[test]
    fn test_parse_meta_file_not_works() {
        let content = "@project(test)\n@path(/home/user/.tip/test/)\n@count(1)\n";
        let tip_meta_got = parse_meta_file(content).expect("Couldn't parse meta file");
        let tip_meta_want = TipMeta {
            project: "@project(test)".to_string(),
            path: "@path(/home/user/.tip/test/)".to_string(),
            count: 1,
        };
        assert_ne!(tip_meta_got, tip_meta_want);
    }
    #[test]
    fn perform_function_tip_works() {
        // Assuming these will not error out.
        match fs::create_dir("temp2") {
            Ok(res) => res,
            Err(error) => match error.kind() {
                std::io::ErrorKind::AlreadyExists => {}
                _ => {
                    panic!("Error: {error:?}")
                }
            },
        }
        let cmd_setup = Command::new("create", "@project", vec![]).expect("setup: failed cmd new");
        let tip_cli_setup = TipCli {
            tip_dir: "./temp2".to_string(),
            project_name: String::from("imgv"),
            command: cmd_setup,
        };
        tip_cli_setup
            .perform()
            .expect("setup: perform(imgv create @project) failed");
        let cmd = Command::new(
            "create",
            "@tip",
            vec!["4".to_string(), "Add file explorer".to_string()],
        )
        .expect("Error: creating new command");
        let tip_cli = TipCli {
            tip_dir: tip_cli_setup.tip_dir.clone(),
            project_name: String::from("imgv"),
            command: cmd,
        };
        let cursor_want = Cursor {
            list: None,
            ok: Ok(()),
        };
        let cursor_got = tip_cli.perform().expect("perform failed");
        assert_eq!(cursor_got, cursor_want);
        fs::remove_dir_all(tip_cli.tip_dir).expect("couldn't remove directory");
    }
}
