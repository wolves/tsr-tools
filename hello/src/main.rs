use hello::world;

fn main() {
    println!("{}", world());
}

#[test]
fn world_returns_hello_world() {
    assert_eq!(world(), "Hello, world!", "wrong message")
}
