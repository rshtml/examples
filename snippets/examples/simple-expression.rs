use rshtml::View;

#[derive(View)]
#[view(path = "views/simple_expression.rs.html")]
struct SimpleExpressionPage {
    value: i32,
    data: Option<String>,
    for_escape: String,
}

impl SimpleExpressionPage {
    fn my_func(&self) -> String {
        "my func".to_string()
    }
}

fn main() {
    let page = SimpleExpressionPage {
        value: 10,
        data: Some("Hello".to_string()),
        for_escape: "'<script/>'".to_string(),
    };

    let mut out = String::with_capacity(page.text_size());

    page.render(&mut out).unwrap();
    println!("{}", out);
}
