use crate::ir::RunProps;

fn escape_xml(s: &str) ->String{
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            c @ _ => out.push(c)
        }
    }
    out
}
fn emit_run(text:&str, rp: RunProps, out:&mut String){
    if text.is_empty(){
        return;
    }
    if rp.has_any(){
        out.push_str("<w:rPr>");
        
    }
}