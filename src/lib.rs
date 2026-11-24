use std::{collections::HashMap, fmt, fs};

pub mod tip_cli;
pub mod types;
pub use tip_cli::TipCli;
pub use types::*;

// TODO: Replace enums with Self where possible
impl FunctionTypes {
    pub const fn to_str(&self) -> &'static str {
        match &self {
            FunctionTypes::All => "@all",
            FunctionTypes::Project => "@project",
            FunctionTypes::Tip => "@tip",
            FunctionTypes::Where => "@where",
        }
    }
    pub fn new(function: &str) -> Result<FunctionTypes, String> {
        match function {
            "@all" => Ok(FunctionTypes::All),
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
            FunctionTypes::All | FunctionTypes::Project => {
                // @project and @all functions accept 0 arguments
                if !args.is_empty() {
                    return Err("Wrong number of arguments provided: expected none.".to_string());
                }
                Ok(Function {
                    func_args: args.clone(),
                    func_type: fn_type,
                })
            }
            FunctionTypes::Tip => {
                // @tip expects at least 2 arguments: $1 = critical $2... = title
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
            FunctionTypes::Where => {
                // @where is going to take max 9 arguments and min 3 arguments.
                if args.len() < 3 && args.len() > 9 {
                    return Err(
                        "Wrong number of arguments provided: expected at least 3 and <= 9"
                            .to_string(),
                    );
                }
                Ok(Function {
                    func_args: args,
                    func_type: fn_type,
                })
            }
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

// TODO: Add method to validate command signature
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
            "@project:{}\n@path:{}\n@count:{}\n",
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
            .map_err(|e| format!("Error: couldn't convert count to u32 {e}"))?;
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
                return Err("Couldn't get title from parsed tip file.".to_string());
            }
        };
        let critical = match parsed.get("critical") {
            Some(p) => p,
            _ => {
                return Err("Couldn't get critical from parsed tip file.".to_string());
            }
        };
        let critical: u8 = critical
            .trim()
            .parse()
            .map_err(|e| format!("Error: couldn't convert critical to u8 {e}"))?;
        let done = match parsed.get("done") {
            Some(c) => c,
            _ => {
                return Err("Couldn't get done from parsed tip file.".to_string());
            }
        };
        let done: bool = done
            .trim()
            .parse()
            .map_err(|e| format!("Error: couldn't convert done to bool {e}"))?;
        Ok(TipFile {
            title: TipColTypes::Title(title.to_string()),
            critical: TipColTypes::Critical(critical),
            done: TipColTypes::Done(done),
            id: TipColTypes::Id(id),
        })
    }
}

impl Value {
    pub fn get_val_string(&self) -> Result<String, String> {
        match self {
            Self::Str(s) => Ok(s.clone()),
            Self::Con(c) => Err(format!("Expected Value::Str(), provided Value::{:?}", c)),
        }
    }
    pub fn get_val_condition(&self) -> Result<Box<Condition>, String> {
        match self {
            Self::Con(c) => Ok(c.clone()),
            Self::Str(s) => Err(format!("Expected Value::Con(), provided Value::{:?}", s)),
        }
    }
}

impl TipColTypes {
    fn new(col: String, val: String) -> Result<Self, String> {
        match col.as_str() {
            "id" => match val.parse::<u32>() {
                Ok(v) => Ok(Self::Id(v)),
                Err(_) => Err("Couldn't convert from String to u32".to_string()),
            },
            "title" => Ok(Self::Title(val)),
            "critical" => match val.parse::<u8>() {
                Ok(v) => Ok(Self::Critical(v)),
                Err(_) => Err("Couldn't convert from String to u8".to_string()),
            },
            "done" => match val.parse::<bool>() {
                Ok(v) => Ok(Self::Done(v)),
                Err(e) => Err(format!("Couldn't convert from String to bool: {e}, {val}")),
            },
            other => Err(format!("Unknown column {other}")),
        }
    }
}

// Adding the Display trait
impl fmt::Display for TipColTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(i) => f.write_str(i.to_string().as_str()),
            Self::Title(t) => f.write_str(t.to_string().as_str()),
            Self::Critical(c) => f.write_str(c.to_string().as_str()),
            Self::Done(d) => f.write_str(d.to_string().as_str()),
        }
    }
}

// Added default trait: Default::default() will still not work
impl Default for Condition {
    fn default() -> Self {
        Self::new()
    }
}

impl Condition {
    pub fn new() -> Self {
        Self {
            operator: String::new(),
            operand_lt: Value::Str(String::new()),
            operand_rt: Value::Str(String::new()),
        }
    }

    fn _compare_internal(val1: TipColTypes, val2: TipColTypes, op: String) -> bool {
        match op.as_str() {
            Self::IS => val1 == val2,
            Self::GT => val2 > val1,
            Self::LT => val2 < val1,
            Self::GE => val2 >= val1,
            Self::LE => val2 <= val1,
            _ => false,
        }
    }

    pub fn compare(expr: Condition, file_parsed: TipFile) -> Result<bool, String> {
        let op_rt_str = expr.operand_rt.get_val_string()?;
        match expr.operand_lt.get_val_string()?.as_str() {
            "id" => match expr.operand_rt.get_val_string() {
                Ok(_) => {
                    let col = TipColTypes::new("id".to_string(), op_rt_str)?;
                    Ok(Self::_compare_internal(
                        col,
                        file_parsed.id.clone(),
                        expr.operator.clone(),
                    ))
                }
                Err(e) => Err(e),
            },
            "title" => match expr.operand_rt.get_val_string() {
                Ok(_) => {
                    let col = TipColTypes::new("title".to_string(), op_rt_str)?;
                    Ok(Condition::_compare_internal(
                        col,
                        file_parsed.title.clone(),
                        expr.operator.clone(),
                    ))
                }
                Err(e) => Err(e),
            },
            "critical" => match expr.operand_rt.get_val_string() {
                Ok(_) => {
                    let col = TipColTypes::new("critical".to_string(), op_rt_str)?;
                    Ok(Condition::_compare_internal(
                        col,
                        file_parsed.critical.clone(),
                        expr.operator.clone(),
                    ))
                }
                Err(e) => Err(e),
            },
            "done" => match expr.operand_rt.get_val_string() {
                Ok(_) => {
                    let col = TipColTypes::new("done".to_string(), op_rt_str)?;
                    Ok(Condition::_compare_internal(
                        col,
                        file_parsed.done.clone(),
                        expr.operator.clone(),
                    ))
                }
                Err(e) => Err(e),
            },
            other => Err(format!("Unknown column {other}")),
        }
    }

    fn parse_condition(content: Vec<String>) -> Condition {
        let mut main_condition = Condition::new();
        for (i, ch) in content.iter().enumerate() {
            if ch == Condition::IS
                || ch == Condition::GT
                || ch == Condition::LT
                || ch == Condition::GE
                || ch == Condition::LE
            {
                main_condition = Condition::make_condition(
                    ch.to_string(),
                    <String as Clone>::clone(&content[i - 1]),
                    <String as Clone>::clone(&content[i + 1]),
                );
            }
        }
        main_condition
    }

    // Not using AST (even though current structure looks a little bit similar). Might change in future.
    pub fn parse_expression(content: Vec<String>) -> Condition {
        let mut start = 0;
        let mut operand_lt = Condition::new();
        let mut operand_rt = Condition::new();
        let mut operator = String::new();
        let main_exprs: Condition;
        for (i, ch) in content.iter().enumerate() {
            if ch == Condition::AND || ch == Condition::OR {
                operator = ch.to_string();
                operand_lt = Self::parse_condition(content[start..i].to_vec().clone());
                start += 1;
            } else if (i == content.len() - 1)
                && (operator == Condition::AND || operator == Condition::OR)
            {
                operand_rt = Self::parse_condition(content);
                main_exprs = Condition::make_expression(operator, operand_lt, operand_rt);
                return main_exprs;
            } else if (i == content.len() - 1)
                && (operator != Condition::AND || operator != Condition::OR)
            {
                main_exprs = Condition::parse_condition(content);
                return main_exprs;
            }
        }
        main_exprs = Condition::make_expression(operator, operand_lt, operand_rt);
        main_exprs
    }

    pub fn make_condition(operator: String, operand_lt: String, operand_rt: String) -> Self {
        Self {
            operator,
            operand_lt: Value::Str(operand_lt),
            operand_rt: Value::Str(operand_rt),
        }
    }

    pub fn make_expression(operator: String, operand_lt: Condition, operand_rt: Condition) -> Self {
        Self {
            operator,
            operand_lt: Value::Con(Box::new(operand_lt)),
            operand_rt: Value::Con(Box::new(operand_rt)),
        }
    }
}

impl Cursor {
    // I should probably sort the list first.
    pub fn print_cursor_list(&self) {
        if let Some(l) = &self.list {
            for t in l {
                println!(
                    "@id:{} | @title:{} | @critical:{} | @done:{}",
                    t.id, t.title, t.critical, t.done
                );
            }
        }
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
            id: TipColTypes::Id(1),
            title: TipColTypes::Title("test".to_string()),
            critical: TipColTypes::Critical(10),
            done: TipColTypes::Done(false),
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
            id: TipColTypes::Id(1),
            title: TipColTypes::Title("test".to_string()),
            critical: TipColTypes::Critical(10),
            done: TipColTypes::Done(false),
        };
        assert_eq!(tip_got, Ok(tip_want));
    }
    #[test]
    fn test_parse_tip_file_not_works_v1() {
        let content = "@title:test\n@critical:10\n@done:false\n";
        let tip_got = ParseFile::parse_tip_file(content, 1);
        let tip_want = TipFile {
            id: TipColTypes::Id(1),
            title: TipColTypes::Title(":test".to_string()),
            critical: TipColTypes::Critical(1),
            done: TipColTypes::Done(true),
        };
        assert_ne!(tip_got, Ok(tip_want));
    }
    #[test]
    fn test_parse_tip_file_not_works_v2() {
        let content = "@title:test\n\n@done:false\n";
        let tip_got = ParseFile::parse_tip_file(content, 1);
        let tip_want = TipFile {
            id: TipColTypes::Id(1),
            title: TipColTypes::Title(":test".to_string()),
            critical: TipColTypes::Critical(1),
            done: TipColTypes::Done(true),
        };
        assert_ne!(tip_got, Ok(tip_want));
    }
    // What does test driven development mean anyways?
    #[test]
    fn test_compare_internal_works() {
        assert!(Condition::_compare_internal(
            TipColTypes::Id(10),
            TipColTypes::Id(10),
            String::from("is")
        ));
    }
    #[test]
    fn test_compare_internal_not_works() {
        assert!(!Condition::_compare_internal(
            TipColTypes::Id(10),
            TipColTypes::Title(String::from("Hello")),
            String::from("is")
        ));
    }
    #[test]
    fn test_compare_works() {
        let con = Condition {
            operator: "is".to_string(),
            operand_lt: Value::Str("critical".to_string()),
            operand_rt: Value::Str("10".to_string()),
        };
        assert_eq!(
            Condition::compare(
                con,
                TipFile {
                    id: TipColTypes::Id(1),
                    title: TipColTypes::Title(String::from("Hello")),
                    critical: TipColTypes::Critical(10),
                    done: TipColTypes::Done(false),
                }
            ),
            Ok(true)
        );
    }
    #[test]
    fn test_compare_not_works() {
        let con = Condition {
            operator: "is".to_string(),
            operand_lt: Value::Str("done".to_string()),
            operand_rt: Value::Str("true".to_string()),
        };
        assert_ne!(
            Condition::compare(
                con,
                TipFile {
                    id: TipColTypes::Id(1),
                    title: TipColTypes::Title(String::from("Hello")),
                    critical: TipColTypes::Critical(10),
                    done: TipColTypes::Done(false),
                }
            ),
            Ok(true)
        );
    }
    #[test]
    fn test_parse_condition_works() {
        let content = vec![
            "title".to_string(),
            "is".to_string(),
            "hello world".to_string(),
        ];
        let want = Condition {
            operator: String::from("is"),
            operand_lt: Value::Str(String::from("title")),
            operand_rt: Value::Str(String::from("hello world")),
        };
        let got = Condition::parse_condition(content);
        assert_eq!(want, got);
    }
    #[test]
    fn test_parse_condition_not_works() {
        let content = vec!["done".to_string(), "is".to_string(), "5".to_string()];
        let want = Condition {
            operator: String::from("done"),
            operand_lt: Value::Con(Box::default()),
            operand_rt: Value::Con(Box::default()),
        };
        let got = Condition::parse_condition(content);
        assert_ne!(want, got);
    }
    #[test]
    fn test_parse_expression_works_v1() {
        let got = Condition::parse_expression(vec![
            "title".to_string(),
            "is".to_string(),
            "hello world".to_string(),
            "and".to_string(),
            "critical".to_string(),
            "is".to_string(),
            "4".to_string(),
        ]);
        let lhs = Box::new(Condition {
            operator: "is".to_string(),
            operand_lt: Value::Str("title".to_string()),
            operand_rt: Value::Str("hello world".to_string()),
        });
        let rhs = Box::new(Condition {
            operator: "is".to_string(),
            operand_lt: Value::Str("critical".to_string()),
            operand_rt: Value::Str("4".to_string()),
        });
        let want = Condition {
            operator: String::from("and"),
            operand_lt: Value::Con(lhs),
            operand_rt: Value::Con(rhs),
        };
        assert_eq!(want, got);
    }
}
