// The main structure representing the entire program
pub struct Program {
    toplevels: Vec<TopLevel>,
}

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
    Const {
        name: String,
        typ: String,
        value: String,
    },
    Protocal(Protocal), // Creates a protocal
    Enum(Enum),         // Creates an enum
}

pub struct FinalExpr {
    line: usize,
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
    fields_funtions: Vec<Function>,
    functions: Vec<Function>,
}

pub struct Enum {
    name: String,
    variants: Vec<(String, Vec<Type>)>, // (variant_name, variant_types)
    pos: Pos,
}

pub struct Implementation {
    type_name: String,
    functions: Vec<Function>,
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
}

pub struct Extension {
    type_name: String,
    functions: Vec<Function>,
}
