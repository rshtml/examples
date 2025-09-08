use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
struct ForPage {
    users: Vec<String>,
}

fn main() {
    let mut page = ForPage {
        users: vec!["Alice".to_string(), "Bob".to_string()],
    };
    println!("{}", page.render().unwrap());
}
