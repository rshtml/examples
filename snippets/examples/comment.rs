use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
struct CommentPage {}

fn main() {
    let page = CommentPage {};

    println!("{}", page.render().unwrap());
}
