use rshtml::View;

#[derive(View)]
#[view(path = "views/parentheses_expression.rs.html")]
struct ParenthesesExpressionPage {
    value: i32,
    data: String,
}

fn main() {
    let page = ParenthesesExpressionPage {
        value: 10,
        data: "Hello".to_string(),
    };

    let mut out = String::with_capacity(page.text_size());

    page.render(&mut out).unwrap();
    println!("{}", out);
}
