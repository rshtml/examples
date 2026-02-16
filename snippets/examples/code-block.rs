use rshtml::View;

#[derive(View)]
#[view(path = "views/code_block.rs.html")]
struct CodeBlockPage {}

fn main() {
    let page = CodeBlockPage {};

    let mut out = String::with_capacity(page.text_size());

    page.render(&mut out).unwrap();
    println!("{}", out);
}
