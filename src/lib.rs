use std::{fmt, fs};

pub mod tip_cli;
pub mod types;
pub use tip_cli::TipCli;
pub use types::*;

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

impl ParseFile {
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
                ParseFile::store_data(&mut project_name, line);
            } else if line.starts_with("@path") {
                ParseFile::store_data(&mut project_path, line);
            } else if line.starts_with("@count") {
                ParseFile::store_data(&mut project_count, line);
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
    fn test_store_data_works() {
        let line = "(FirstTest)";
        let mut var = String::new();
        ParseFile::store_data(&mut var, line);
        assert_eq!(var, "FirstTest");
    }
    #[test]
    fn test_store_data_not_works() {
        let line = "(SecondTest)";
        let mut var = String::new();
        ParseFile::store_data(&mut var, line);
        assert_ne!(var, "FirstTest");
    }
    #[test]
    fn test_parse_meta_file_works() {
        let content = "@project(test)\n@path(/home/user/.tip/test/)\n@count(1)\n";
        let tip_meta_got = ParseFile::parse_meta_file(content).expect("Couldn't parse meta file");
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
        let tip_meta_got = ParseFile::parse_meta_file(content).expect("Couldn't parse meta file");
        let tip_meta_want = TipMeta {
            project: "@project(test)".to_string(),
            path: "@path(/home/user/.tip/test/)".to_string(),
            count: 1,
        };
        assert_ne!(tip_meta_got, tip_meta_want);
    }
}
