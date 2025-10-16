// The main structure representing the entire program
struct Program<'a> {
    toplevels: Vec<TopLevel<'a>>,
}

enum DefaultTypes {
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

enum TopLevel<'a> {
    Mod(Module<'a>),    // Module definition
    Typ(Type<'a>),      // Type definition
    Ext(Extension<'a>), // Create an type or enum extension by implements protocal
    Imp(&'a str),       // Importing modules or types
    Function(Function<'a>),
    Const {
        name: &'a str,
        typ: &'a str,
        value: &'a str,
    },
    Static {
        name: &'a str,
        typ: &'a str,
        value: &'a str,
    },
    Protocal(Protocal<'a>), // Creates a protocal
    Enum(Enum<'a>),         // Creates an enum
}

struct FinalExpr {
    line: usize,
}

struct Pos {
    line: usize,
    col: usize,
    file: String,
}

struct Function<'a> {
    name: String,
    args: Vec<Arg<'a>>,
    body: FinalExpr,
    ret_type: Option<String>,
    pos: Pos,
}

struct Arg<'a> {
    name: &'a str,
    type_: String,              // Type of the argument
    pos: Pos,                   // Position in the source Module
    default: Option<FinalExpr>, // Optional default value for the argument
}

struct Type<'a> {
    name: String,
    fields: Vec<(String, Type<'a>)>, // (field_name, field_type)
    pos: Pos,
    fields_funtions: Vec<Function<'a>>,
    functions: Vec<Function<'a>>,
}

struct Enum<'a> {
    name: String,
    variants: Vec<(String, Vec<Type<'a>>)>, // (variant_name, variant_types)
    pos: Pos,
}

struct Implementation<'a> {
    type_name: String,
    functions: Vec<Function<'a>>,
    pos: Pos,
}

struct Protocal<'a> {
    // similar to traits in Rust
    name: String,
    functions: Vec<Function<'a>>,
    pos: Pos,
}

struct Module<'a> {
    name: &'a str,
}

struct Extension<'a> {
    type_name: &'a str,
    functions: Vec<Function<'a>>,
}
