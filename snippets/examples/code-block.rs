use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
#[rshtml(path = "code_block.rs.html")]
struct CodeBlockPage {}

fn main() {
    let mut page = CodeBlockPage {};
    println!("{}", page.render().unwrap());
}
