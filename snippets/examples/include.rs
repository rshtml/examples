use rshtml::{traits::RsHtml, RsHtml};

#[derive(RsHtml)]
struct IncludePage {
    value: i32,
    data: String,
}

impl IncludePage {
    fn my_func(&self) -> String {
        let mut hold = "Func".to_string();
        hold.push_str(self.data.clone().as_str());
        hold
    }
}

fn main() {
    let mut page = IncludePage {
        value: 10,
        data: "Hello".to_string(),
    };

    println!("{}", page.render().unwrap());
}
