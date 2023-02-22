fn main() {
    let v1 = [1, 2, 3];
    let mut iter = v1.into_iter();

    loop {
        match iter.next() {
            Some(i) => {
                println!("{}", i);
            }
            None => break,
        }
    }

    let mut iter = v1.into_iter();
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(3), iter.next());
    assert_eq!(None, iter.next());
}
