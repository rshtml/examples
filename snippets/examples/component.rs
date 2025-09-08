use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
struct ComponentPage {
    value: i32,
    title: String,
    data: String,
    for_escape: String,
}

fn main() {
    let mut page = ComponentPage {
        value: 10,
        title: "Component".to_string(),
        data: "Hello".to_string(),
        for_escape: "'<script/>'".to_string(),
    };

    println!("{}", page.render().unwrap());
}
