use rshtml::View;

#[derive(View)]
struct ComponentPage {
    value: i32,
    title: String,
    data: String,
    for_escape: String,
    items: Vec<Item>,
}

struct Item {
    name: String,
}

fn main() {
    let page = ComponentPage {
        value: 10,
        title: "Component".to_string(),
        data: "Hello".to_string(),
        for_escape: "'<script/>'".to_string(),
        items: vec![
            Item {
                name: "Jack".to_string(),
            },
            Item {
                name: "John".to_string(),
            },
        ],
    };

    let mut out = String::with_capacity(page.text_size());

    page.render(&mut out).unwrap();
    println!("{}", out);
}
