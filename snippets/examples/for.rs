use rshtml::View;

#[derive(View)]
struct ForPage {
    users: Vec<String>,
}

fn main() {
    let page = ForPage {
        users: vec!["Alice".to_string(), "Bob".to_string()],
    };

    let mut out = String::with_capacity(page.text_size());
    page.render(&mut out).unwrap();

    println!("{}", out);
}
