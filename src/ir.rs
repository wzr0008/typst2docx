#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    Text(String),
    Strong(Vec<Inline>),
    Emph(Vec<Inline>),
    Code(String),
    Link { href: String, content: Vec<Inline> },
    Math(String),
    LineBreak,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Heading { level: u8, inlines: Vec<Inline> },
    Paragraph(Vec<Inline>),
    List(List),
    Table(Table),
    DisplayMath(String),
    CodeBlock(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub ordered: bool,
    pub items: Vec<ListItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub blocks: Vec<Block>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub rows: Vec<Row>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub is_header: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub col_span: u32,
    pub row_span: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct RunProps {
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub link: bool,
}
impl RunProps {
    fn with_bold(&self) -> RunProps {
        RunProps {
            bold: true,
            code: self.code,
            italic: self.italic,
            link: self.link,
        }
    }
    fn with_italic(&self) -> RunProps {
        RunProps {
            italic: true,
            bold: self.bold,
            code: self.code,
            link: self.link,
        }
    }
    fn with_code(&self) -> RunProps {
        RunProps {
            code: true,
            bold: self.bold,
            italic: self.italic,
            link: self.link,
        }
    }
    fn with_link(&self) -> RunProps {
        RunProps {
            link: true,
            bold: self.bold,
            italic: self.italic,
            code: self.code,
        }
    }
    pub fn has_any(&self) -> bool {
        self.italic || self.code || self.link || self.bold
    }
}
