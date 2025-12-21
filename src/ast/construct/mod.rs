// The main structure representing the entire program
pub struct Program {
    toplevels: Vec<TopLevel>,
}

// Default types in the program
pub enum DefaultTypes {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    String,
    Bool,
    F32,
    F64,
    Char,
    Unit, // represents the unit type `()`
}

pub enum TopLevel {
    Mod(Module),    // Module definition
    Typ(Type),      // Type definition
    Ext(Extension), // Create an type or enum extension by implements protocal
    Imp(String),    // Importing modules or types
    Function(Function),
    Const(Const),
    Protocal(Protocal), // Creates a protocal
    Enum(Enum),         // Creates an enum
}

pub struct FinalExpr {
    exprs: Vec<Expr>,
}

pub enum Expr {
    FunctionCall { f: Function },
    BultinFunctionCall { f: BultinFunction },
    Constant { v: Const },
}

pub struct Pos {
    line: usize,
    col: usize,
    file: String,
}

pub struct Function {
    name: String,
    args: Vec<Arg>,
    body: FinalExpr,
    ret_type: Option<String>,
    pos: Pos,
}

pub struct Const {
    name: String,
    typ: String,
    value: String,
}

// TODO: add builtin functions here (exp. add, sub, mul, div)
pub enum BultinFunction {}

pub struct Arg {
    name: String,
    type_: String,              // Type of the argument
    pos: Pos,                   // Position in the source Module
    default: Option<FinalExpr>, // Optional default value for the argument
}

pub struct Type {
    name: String,
    fields: Vec<(String, Type)>, // (field_name, field_type)
    pos: Pos,
    fields_funtions: Vec<String>, // function in `Implementation` will have "parent" etc. Cat::meow
    functions: Vec<Function>,
}

pub struct Enum {
    name: String,
    variants: Vec<(String, Vec<String>)>, // (variant_name, variant_types)
    pos: Pos,
}

pub struct Implementation {
    type_name: String,
    functions: String, // function in `Implementation` will have "parent" etc. Cat::meow
    pos: Pos,
}

pub struct Protocal {
    // similar to traits in Rust
    name: String,
    functions: Vec<Function>,
    pos: Pos,
}

pub struct Module {
    name: String,
    start_in: String,
}

pub struct Extension {
    type_name: String,
    functions: Vec<Function>,
}
