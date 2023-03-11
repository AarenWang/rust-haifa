fn main() {
    let arr = [1, 2, 3, 4, 5]; //数组，栈上分配内存，不可变
    let mut arr2: [i32; 5] = [1, 2, 3, 4, 5]; //显式定义数组元素数据类型和元素个数

    let v: Vec<i32> = Vec::new();

    //或者使用vec!宏
    let v: Vec<i32> = vec![];

    let v = vec![1, 2, 3, 4, 5];

    //长度为10，元素值为0
    let v = vec![0; 10]; // ten zeroes

    //通过下标访问
    let mut v = vec![1, 2, 3];

    let two = v[1];
    let three: &i32 = &v[2];

    // 改变第一个元素的值
    v[0] = two + three;
}
