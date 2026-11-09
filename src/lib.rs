use std::{collections::HashMap, fmt, fs};

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
            other => Err(format!("Error: invalid function: {other}")),
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
                if args.len() < 2 {
                    return Err(
                        "Wrong number of arguments provided: expected at least 2".to_string()
                    );
                }
                let critical: u8 = args[0].trim().parse().map_err(|_| {
                    "Wrong datatype: critical(first argument) must be an unsigned integer"
                        .to_string()
                })?;
                if !(1..=10).contains(&critical) {
                    return Err("Value of critical can only be between 1 and 10.".to_string());
                }
                Ok(Function {
                    func_args: args,
                    func_type: fn_type,
                })
            }
            other => Err(format!("Not implimented: {other}")),
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
            other => Err(format!("Error: invalid command: {other}")),
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
        let path = format!("{}/000.tip", tip_cli.get_full_path());
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
    fn parse_file(content: &str) -> HashMap<String, String> {
        let mut values = HashMap::new();
        let mut buffer_val = String::new();
        let mut buffer_var = String::new();
        let mut is_var = false;
        let mut is_val = false;
        for ch in content.chars() {
            match ch {
                ParseFile::TOKEN_VARIABLE => {
                    if is_val {
                        values.insert(buffer_var.trim().to_string(), buffer_val.trim().to_string());
                    }
                    if !is_var {
                        is_var = true;
                        is_val = false;
                        buffer_var.clear();
                    }
                }
                ParseFile::TOKEN_ASSIGNMENT => {
                    if !is_val {
                        is_val = true;
                        is_var = false;
                        buffer_val.clear();
                    }
                }
                '\n' => {
                    values.insert(buffer_var.clone(), buffer_val.clone());
                }
                c => {
                    if is_var {
                        assert!(!is_val, "is_var and is_val are both true.");
                        buffer_var.push(c);
                    } else if is_val {
                        assert!(!is_var, "is_val and is_var are both true.");
                        buffer_val.push(c);
                    }
                }
            }
        }
        println!("{:?}", values);
        values
    }

    fn parse_meta_file(content: &str) -> Result<TipMeta, String> {
        let parsed = ParseFile::parse_file(content);
        let project = match parsed.get("project") {
            Some(p) => p,
            _ => {
                return Err("Couldn't get project from parsed metadata file.".to_string());
            }
        };
        let path = match parsed.get("path") {
            Some(p) => p,
            _ => {
                return Err("Couldn't get path from parsed metadata file.".to_string());
            }
        };
        let count = match parsed.get("count") {
            Some(c) => c,
            _ => {
                return Err("Couldn't get count from parsed metadata file.".to_string());
            }
        };
        let count: u32 = count
            .trim()
            .parse()
            .map_err(|e| format!("Error: couldn't convert to u32 {e}"))?;
        Ok(TipMeta {
            project: project.to_string(),
            path: path.to_string(),
            count,
        })
    }

    fn parse_tip_file(content: &str, id: u32) -> Result<TipFile, String> {
        let parsed = ParseFile::parse_file(content);
        let title = match parsed.get("title") {
            Some(p) => p,
            _ => {
                return Err("Couldn't get title".to_string());
            }
        };
        let critical = match parsed.get("critical") {
            Some(p) => p,
            _ => {
                return Err("Couldn't get critical".to_string());
            }
        };
        let critical: u8 = critical.trim().parse().map_err(|e| format!("Error: {e}"))?;
        let done = match parsed.get("done") {
            Some(c) => c,
            _ => {
                return Err("Couldn't get done".to_string());
            }
        };
        let done: bool = done.trim().parse().map_err(|e| format!("Error: {e}"))?;
        Ok(TipFile {
            title: title.to_string(),
            critical,
            done,
            id,
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
    fn test_parse_file_works_v1() {
        let mut parsed_want = HashMap::new();
        let parsed_got = ParseFile::parse_file("@project:test\n");
        parsed_want.insert(String::from("project"), String::from("test"));
        assert_eq!(parsed_got, parsed_want);
    }
    #[test]
    fn test_parse_file_works_v2() {
        let content = "@\ntitle:test\n\n@ critical\n :\n 10\n\n@done:\nfalse\n\n";
        let tip_got = ParseFile::parse_tip_file(content, 1);
        let tip_want = TipFile {
            id: 1,
            title: "test".to_string(),
            critical: 10,
            done: false,
        };
        assert_eq!(tip_got, Ok(tip_want));
    }
    #[test]
    fn test_parse_file_not_works() {
        let mut parsed_want = HashMap::new();
        let parsed_got = ParseFile::parse_file("@project:test\n");
        parsed_want.insert(String::from("count"), String::from("1"));
        assert_ne!(parsed_got, parsed_want);
    }
    #[test]
    fn test_parse_meta_file_works() {
        let content = "@project:test\n@path:/home/user/.tip/test/\n@count:1\n";
        let tip_meta_got = ParseFile::parse_meta_file(content).expect("Couldn't parse meta file");
        let tip_meta_want = TipMeta {
            project: "test".to_string(),
            path: "/home/user/.tip/test/".to_string(),
            count: 1,
        };
        assert_eq!(tip_meta_got, tip_meta_want);
    }
    #[test]
    fn test_parse_meta_file_not_works_v1() {
        let content = "@project:test\n@path:/home/user/.tip/test/\n@count:1\n";
        let tip_meta_got = ParseFile::parse_meta_file(content).expect("Couldn't parse meta file");
        let tip_meta_want = TipMeta {
            project: "@project:test".to_string(),
            path: "@path:/home/user/.tip/test/".to_string(),
            count: 1,
        };
        assert_ne!(tip_meta_got, tip_meta_want);
    }
    #[test]
    fn test_parse_meta_file_not_works_v2() {
        let content = "@project:test\n@count:1\n";
        let tip_meta_got = ParseFile::parse_meta_file(content);
        let tip_meta_want = TipMeta {
            project: "@project:test".to_string(),
            path: "@path:/home/user/.tip/test/".to_string(),
            count: 1,
        };
        assert_ne!(tip_meta_got, Ok(tip_meta_want));
    }
    #[test]
    fn test_parse_tip_file_works_v1() {
        let content = "@title:test\n@critical:10\n@done:false\n";
        let tip_got = ParseFile::parse_tip_file(content, 1);
        let tip_want = TipFile {
            id: 1,
            title: "test".to_string(),
            critical: 10,
            done: false,
        };
        assert_eq!(tip_got, Ok(tip_want));
    }
    #[test]
    fn test_parse_tip_file_not_works_v1() {
        let content = "@title:test\n@critical:10\n@done:false\n";
        let tip_got = ParseFile::parse_tip_file(content, 1);
        let tip_want = TipFile {
            id: 1,
            title: ":test".to_string(),
            critical: 1,
            done: true,
        };
        assert_ne!(tip_got, Ok(tip_want));
    }
    #[test]
    fn test_parse_tip_file_not_works_v2() {
        let content = "@title:test\n\n@done:false\n";
        let tip_got = ParseFile::parse_tip_file(content, 1);
        let tip_want = TipFile {
            id: 1,
            title: ":test".to_string(),
            critical: 1,
            done: true,
        };
        assert_ne!(tip_got, Ok(tip_want));
    }
}
