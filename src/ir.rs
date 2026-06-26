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
    Heading{level: u8, inlines: Vec<Inline>},
    Paragraph(Vec<Inline>),
    List(List),
    Table(Table),
    DisplayMath(String),
    CodeBlock(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct List{
    pub ordered: bool,
    pub items: Vec<ListItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem{
    pub blocks: Vec<Block>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Table{
    pub rows: Vec<Row>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Row{
    pub cells: Vec<Cell>,
    pub is_header: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Cell{
    pub col_span:u32,
    pub row_span:u32,
    pub blocks: Vec<Block>,
}

