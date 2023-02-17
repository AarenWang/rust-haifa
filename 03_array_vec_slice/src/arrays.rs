fn main() {
    let arr = [1, 2, 3, 4, 5]; //数组，栈上分配内存，不可变
    let mut arr2: [i32; 5] = [1, 2, 3, 4, 5]; //显式定义数组元素数据类型和元素个数
    let vec = vec![1, 2, 3, 4, 5]; //向量
    let s1 = &arr[..2];
    let s2 = &vec[..2];
    println!("s1: {:?}, s2: {:?}", s1, s2);

    // &[T] 和 &[T] 是否相等取决于长度和内容是否相等
    assert_eq!(s1, s2);
    // &[T] 可以和 Vec<T>/[T;n] 比较，也会看长度和内容
    assert_eq!(&arr[..], vec);
    assert_eq!(&vec[..], arr);
}
