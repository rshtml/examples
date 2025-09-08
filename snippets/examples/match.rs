use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
struct MatchPage {
    value: i32,
    data: Option<String>,
}

fn main() {
    let mut page = MatchPage {
        value: 10,
        data: Some("Hello".to_string()),
    };

    println!("{}", page.render().unwrap());
}
