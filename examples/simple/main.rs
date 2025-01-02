use std::fs::write;
use crml::{template, Template};

#[template("index")]
pub(crate) struct TestProps {
    a: i32,
}

#[template("other")]
pub(crate) struct OtherProps {
    c: i32,
}

fn main() {
    println!("{}", "saved to ./simple.html");
    write("./simple.html", TestProps { a: 1 }.render()).expect("failed to write file");
}
