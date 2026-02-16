use rshtml::View;

#[derive(View)]
#[view(path = "views/if_else.rs.html")]
struct IfElsePage {
    is_ok: bool,
    count: i32,
}

fn main() {
    let page = IfElsePage {
        is_ok: true,
        count: 10,
    };

    let mut out = String::with_capacity(page.text_size());

    page.render(&mut out).unwrap();
    println!("{}", out);
}
