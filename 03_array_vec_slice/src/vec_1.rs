fn main() {
    //vec!宏进行初始化
    let _v1 = vec![1, 3, 5];

    //Vec::new进行初始化
    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(3);

    //Vec::new进行初始化  指定类型
    let v3: Vec<i32> = vec![];

    //https://doc.rust-lang.org/std/vec/struct.Vec.html
    //https://doc.rust-lang.org/std/vec/index.html
}
