use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
#[rshtml(path = "simple_expression.rs.html")]
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
    let mut page = SimpleExpressionPage {
        value: 10,
        data: Some("Hello".to_string()),
        for_escape: "'<script/>'".to_string(),
    };
    println!("{}", page.render().unwrap());
}
