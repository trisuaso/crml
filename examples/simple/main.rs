mod crml;
use std::fs::write;

pub(crate) struct TestProps {
    a: i32,
}

fn main() {
    println!("{}", "saved to ./simple.html");
    write("./simple.html", crml::index(TestProps { a: 1 })).expect("failed to write file")
}
