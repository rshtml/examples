use rshtml::traits::RsHtml;
use rshtml::RsHtml;

#[derive(RsHtml)]
#[rshtml(path = "if_else.rs.html")]
struct IfElsePage {
    is_ok: bool,
    count: i32,
}

fn main() {
    let mut page = IfElsePage {
        is_ok: true,
        count: 10,
    };

    let content = page.render().unwrap();
    println!("{}", content);
}
