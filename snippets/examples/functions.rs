use rshtml::{View, functions::*};

#[derive(View)]
struct FunctionsPage {
    date: chrono::DateTime<chrono::Utc>,
    users: Vec<String>,
}

fn main() {
    let page = FunctionsPage {
        date: chrono::Utc::now(),
        users: vec!["Alice".to_string(), "Bob".to_string(), "John".to_string()],
    };

    let mut out = String::with_capacity(page.text_size());

    page.render(&mut out).unwrap();
    println!("{}", out);
}
