fn main() {
    let s = "Hello".to_string();
    let join = |i: &str| s + i;
    assert_eq!("Hello World", join(" World"));
    println!("{:?}", s);
}
