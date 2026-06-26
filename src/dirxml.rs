use crate::ir::Block::Heading;
use crate::ir::{Block, Document};

pub fn document_xml(doc: &Document) -> String {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let w = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    let m = "http://schemas.openxmlformats.org/officeDocument/2006/math";
    let r = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    let part = format!(
        "<w:document xmlns:w=\"{}\" xmlns:m=\"{}\" xmlns:r=\"{}\">
  <w:body>",
        w, m, r
    );
    out.push_str(&part);
    for block in &doc.blocks {
        render_block(block, &mut out);
    }
    out.push_str(
        r#"<w:sectPr><w:pgSz w:w="11906" w:h="16838"/><w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440"/></w:sectPr>"#,
    );
    out.push_str("</w:body></w:document>");
    return out;
}
pub fn render_block(block: &Block, out: &mut String) {
    match block {
        Heading { level, inlines } => {
            let lvl = (*level).clamp(1,6);
            out.push_str(
                format!("<w:p><w:pPr><w:pStyle w:val=\"Heading{}\"/></w:pPr>", lvl).as_str(),
            );
            render_inlines(inlines, RunProps::default(), out);
            out.push_str("</w:p>");
        }
        Block::Paragraph(paragraph) => {
            out.push_str("<w:p>");
            render_inlines(paragraph, RunProps::default(), out);
            out.push_str("</w:p>");
        }
        Block::List(list) => todo!(),
        Block::Table(table)=>todo!(),
        
    }
}
