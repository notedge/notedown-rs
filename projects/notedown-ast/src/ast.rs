use num::{BigInt, BigUint};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AST {
    Program(Vec<AST>),
    Suite(Vec<AST>),
    /// - `EmptyStatement`: Skip
    EmptyStatement,
    /// - `ImportStatement`
    ImportStatement {
        data: ImportStatement,
        annotations: Option<Box<AST>>,
    },
    IfStatement {
        pairs: Vec<(AST, AST)>,
        default: Option<Box<AST>>,
        annotations: Option<Box<AST>>,
    },
    LetBinding {
        symbol: Box<AST>,
        modifiers: Vec<String>,
        types: Box<AST>,
        annotations: Option<Box<AST>>,
    },
    /// - `Expression`
    Expression {
        base: Box<AST>,
        eos: bool,
        pos: Position,
        annotations: Option<Box<AST>>,
    },
    /// - `Expression`
    TypeExpression {
        base: Box<AST>,
        pos: Position,
        annotations: Option<Box<AST>>,
    },
    /// - `UnaryOperators`
    ///     - `base`
    UnaryOperators {
        base: Box<AST>,
        prefix: Vec<String>,
        suffix: Vec<String>,
        pos: Position,
    },
    /// - `InfixOperators`
    InfixOperators {
        o: String,
        lhs: Box<AST>,
        rhs: Box<AST>,
        pos: Position,
    },
    ///
    ListExpression(Vec<AST>),
    ///
    TupleExpression(Vec<AST>),
    /// - `SliceExpression`
    /// the terms must `IndexExpression`
    SliceExpression {
        base: Box<AST>,
        list: Vec<AST>,
    },
    IndexExpression(IndexExpression),
    ApplyExpression {
        base: Box<AST>,
        types: Vec<AST>,
        args: Vec<AST>,
        kv_pairs: Vec<(AST, AST)>,
        pos: Position,
    },
    /// - `Symbol`
    Symbol {
        name: String,
        scope: Vec<String>,
    },
    /// - `Number`: raw number represent
    NumberLiteral {
        handler: String,
        data: String,
    },
    ///
    Number(Number),
    ///
    StringLiteral {
        handler: String,
        data: String,
    },
    /// - `String`: raw string
    String(String),
    /// - `Comment`: raw comment with handler
    CommentLiteral {
        handler: String,
        data: String,
    },
    ///
    Boolean(bool),
    /// - `None`: It doesn't look like anything to me
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImportStatement {
    Local { root: u8, path: Vec<String> },
    LocalAlias { root: u8, path: Vec<String>, alias: String },
    URL { path: String },
    URLAlias { path: String, alias: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IndexExpression {
    None,
    ///
    Single(),
    Normal {
        start: Box<AST>,
        end: Box<AST>,
        step: Box<AST>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Number {
    Integer(BigInt),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Integer128(i128),
    Integer256(String),
    Unsigned(BigUint),
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Unsigned128(u128),
    Unsigned256(String),
    Decimal(String),
    Decimal32(f32),
    Decimal64(f64),
}
