#[derive(Debug, Clone, PartialEq)]
pub enum MathNode {
    Row(Vec<MathNode>),
    Ident {
        name: String,
        italic: bool,
    },
    Number(String),
    Operator(String),
    Text(String),
    Space,
    Fraction {
        num: Box<MathNode>,
        den: Box<MathNode>,
        bar: bool,
    },
    Scripted {
        base: Box<MathNode>,
        sub: Option<Box<MathNode>>,
        sup: Option<Box<MathNode>>,
    },
    UnderOver {
        base: Box<MathNode>,
        under: Option<Box<MathNode>>,
        over: Option<Box<MathNode>>,
        kind: OverUnderKind,
    },
    Root {
        radicand: Box<MathNode>,
        degree: Option<Box<MathNode>>,
    },
    Fenced {
        open: String,
        close: String,
        body: Box<MathNode>,
    },
    BigOp {
        op: String,
        sub: Option<Box<MathNode>>,
        sup: Option<Box<MathNode>>,
        body: Box<MathNode>,
    },
    Matrix {
        rows: Vec<Vec<MathNode>>,
    },
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OverUnderKind {
    Limit,
    Accent,
    Bar,
    Stretch,
}
