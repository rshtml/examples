use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
struct ExtendsPage {
    value: i32,
    data: String,
    for_escape: String,
}

impl ExtendsPage {
    fn my_func(&self) -> String {
        let mut hold = "Func".to_string();
        hold.push_str(self.data.clone().as_str());
        hold
    }
}

fn main() {
    let mut page = ExtendsPage {
        value: 10,
        data: "Hello".to_string(),
        for_escape: "'<script/>'".to_string(),
    };

    println!("{}", page.render().unwrap());
}
