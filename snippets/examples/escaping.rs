use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
struct EscapingPage {
    my_var: String,
}

fn main() {
    let mut page = EscapingPage {
        my_var: "<p>This is <strong>bold</strong> text.</p>".to_string(),
    };

    println!("{}", page.render().unwrap());
}
