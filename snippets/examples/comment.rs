use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
struct CommentPage {}

fn main() {
    let mut page = CommentPage {};

    println!("{}", page.render().unwrap());
}
