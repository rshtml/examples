use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
#[rshtml(path = "parentheses_expression.rs.html")]
struct ParenthesesExpressionPage {
    value: i32,
    data: String,
}

fn main() {
    let mut page = ParenthesesExpressionPage {
        value: 10,
        data: "Hello".to_string(),
    };

    println!("{}", page.render().unwrap());
}
