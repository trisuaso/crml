mod crml;

pub(crate) struct TestProps {
    a: i32,
}

fn main() {
    println!("rendered: {}", crml::index(TestProps { a: 1 }))
}
