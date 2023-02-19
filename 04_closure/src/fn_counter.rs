// compile error:  can't capture dynamic environment in a fn item  help: use the `|| { ... }` closure form instead

// fn counter(i: i32) -> fn(i32) -> i32{
//     fn inc(n: i32) -> i32 {
//         n + i
//     }
//     inc
// }

fn counter(i: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |n: i32| n + i)
}

fn five() -> i32 {
    5
}

fn main() {
    //let f = counter(2);
    let f = counter(2);
    assert_eq!(3, f(1));
    assert_eq!(4, f(2));
    

    let add = |a: i32, b: i32| -> i32 { a + b };
    assert_eq!(3, add(1, 2));

    // 闭包基本结构
    // let clousre_name = |参数名称1:参数类型,参数名称2:参数类型,参数名称N:参数类型| -> { 闭包函数体 }
    //闭包函数参数类型和函数参数类型一样，可以省略声明参数类型

    //闭包参数可以为一个闭包
    let add2 = |a: fn() -> i32, b: i32, c: i32| (a)() + b + c;
    let r = add2(five, 2, 3);
    let r = add2(|| 5, 2, 3);
    assert_eq!(10, r);

    // let c1: () = || { println! ("i am a closure") };
    
}
