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

#[derive(Debug, Clone, PartialEq)]
pub struct TipFile {
    pub id: u32,
    pub title: String,
    pub critical: u8,
    pub done: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cursor {
    pub list: Option<Vec<TipFile>>,
    pub ok: Result<(), String>,
}
