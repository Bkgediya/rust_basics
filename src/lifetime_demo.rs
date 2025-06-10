fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

struct User<'a> {
    name: &'a str,
}

pub fn run() {
    let ans;
    let str1 = String::from("small");
    {
        let str2 = String::from("longest");
        ans = longest(&str1, &str2);
    }

    let name = String::from("Hey");
    let user = User { name: &name };
}
