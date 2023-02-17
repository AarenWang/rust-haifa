fn main() {
    let string1 = String::from("hello world");
    let slice1 = &string1[0..6];
    println!("{}", slice1);

    let a1 = [1, 2, 3, 4, 5, 6];
    let slice2 = &a1[2..4];
    assert_eq!([3, 4], slice2);
}
