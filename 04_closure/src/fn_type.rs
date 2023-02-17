//#! [feature(fn_traits)]
fn main() {
    let s = "hello"; // 声明了变量绑定s为字符串字面量，其为复制语义类型
    let c = || println!("{:?}", s); // 闭包c会按照不可变引用来捕获s

    c();
    c();
    //c.call_mut(()); fn_traits 打开可以调用
    //c.call_once(());
    println!("{:?}", s); // 语句会对s进行一次不可变借用，这就证闭包对s进行了不可变借用

    //移动语义类型自动实现FnOnce
    // closure cannot be invoked more than once because it moves the variable `str1` out of its environment
    let str1 = "hello".to_string();
    let c1 = || str1;

    c1();
    //c1();  // this value implements `FnOnce`, which causes it to be moved when called
    //println!("{:?}",str1);
}
