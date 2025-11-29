pub fn print() {
    println!("Hello, world!");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b + 1
}

#[test]
fn add_2_and_2_returns_4() {
    let result = add(2, 2);
    assert_eq!(result, 4, "add(2, 2): want 4, got {result}");
}
