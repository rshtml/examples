use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
#[rshtml(path = "raw_block.rs.html")]
struct RawBlockPage {}

fn main() {
    let mut page = RawBlockPage {};

    println!("{}", page.render().unwrap());
}
