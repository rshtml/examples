use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
struct WhilePage {
    count: i32,
}

impl WhilePage {
    fn increment(&mut self) -> String {
        self.count += 1;
        "".to_string()
    }
}

fn main() {
    let mut page = WhilePage { count: 1 };
    println!("{}", page.render().unwrap());
}
