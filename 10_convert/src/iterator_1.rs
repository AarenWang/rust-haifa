fn main() {
    let a = [1, 2, 3];

    let mut iter = a.iter();

    // A call to next() returns the next value...
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());

    // ... and then None once it's over.
    assert_eq!(None, iter.next());

    // More calls may or may not return `None`. Here, they always will.
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());

    let a = [1, 2, 3];
    let iter = a.iter();

    assert_eq!((3, Some(3)), iter.size_hint());

    // The even numbers in the range of zero to nine.
    let iter = (0..10).filter(|x| x % 2 == 0);

    // We might iterate from zero to ten times. Knowing that it's five
    // exactly wouldn't be possible without executing filter().
    assert_eq!((0, Some(10)), iter.size_hint());

    // Let's add five more numbers with chain()
    let iter = (0..10).filter(|x| x % 2 == 0).chain(15..20);

    // now both bounds are increased by five
    assert_eq!((5, Some(15)), iter.size_hint());
}
