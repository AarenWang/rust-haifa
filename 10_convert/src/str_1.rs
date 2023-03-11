fn main() {
    //转为u8数组
    let bytes1 = "abcdABCD".as_bytes();
    //字节数组长度为8
    assert_eq!(8, bytes1.len());
    println!("{}", bytes1[1]); //字节数组第二个元素是98

    //转为Bytes，Bytes类型为An iterator over the bytes of a string slice.
    let bytes2 = "abcdABCD".bytes();
    //将每个字符对应U8表示打印出来(ASCII码)
    for b in bytes2 {
        print!("{} \t", b);
    }

    //将u8类型数组
    let bytes3 = vec![97u8, 98u8, 99u8, 100u8, 65u8, 66u8, 67u8, 68u8];
    let string1 = String::from_utf8(bytes3);
    assert_eq!("abcdABCD", string1.unwrap());

    // 使用into()
    let str1: String = "Hello".into();
}
