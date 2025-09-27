use rshtml::{functions::*, traits::RsHtml, RsHtml};

#[derive(RsHtml)]
struct FunctionsPage {
    date: chrono::DateTime<chrono::Utc>,
    users: Vec<String>,
}

fn main() {
    let mut page = FunctionsPage {
        date: chrono::Utc::now(),
        users: vec!["Alice".to_string(), "Bob".to_string(), "John".to_string()],
    };

    println!("{}", page.render().unwrap());
}
