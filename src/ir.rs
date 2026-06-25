pub(crate) mod ir;
#[derive(Debug, Clone, PartialEq)]
pub enum Inline{
    Text(String),
    Strong(Vec<Inline>),
    Emph(Vec<Inline>),
    Code(String),
    Link{ href:String, content:Vec<Inline>},
    Math(String),
    LineBreak,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Document{
    pub blocks: Vec<Block>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Block{

}