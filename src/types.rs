#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FunctionTypes {
    All,
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

pub struct ParseFile;

impl ParseFile {
    pub const TOKEN_VARIABLE: char = '@';
    pub const TOKEN_ASSIGNMENT: char = ':';
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
pub struct TipMeta {
    pub project: String,
    pub path: String,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TipColTypes {
    Id(u32),
    Title(String),
    Critical(u8),
    Done(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TipFile {
    pub id: TipColTypes,
    pub title: TipColTypes,
    pub critical: TipColTypes,
    pub done: TipColTypes,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Con(Box<Condition>),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub operator: String,
    pub operand_lt: Value,
    pub operand_rt: Value,
}

// No negative conditions possible, add `NOT`
impl Condition {
    pub const AND: &str = "and";
    pub const OR: &str = "or";
    pub const IS: &str = "is";
    pub const GT: &str = "gt";
    pub const LT: &str = "lt";
    pub const LE: &str = "le";
    pub const GE: &str = "ge";
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cursor {
    pub list: Option<Vec<TipFile>>,
    pub ok: Result<(), String>,
}
