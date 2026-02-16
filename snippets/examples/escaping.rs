use rshtml::View;

#[derive(View)]
struct EscapingPage {
    my_var: String,
}

fn main() {
    let page = EscapingPage {
        my_var: "<p>This is <strong>bold</strong> text.</p>".to_string(),
    };

    let mut out = String::with_capacity(page.text_size());

    page.render(&mut out).unwrap();
    println!("{}", out);
}
