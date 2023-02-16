fn main(){

    let hello1 = "Hello, world!";

    // with an explicit type annotation
    let hello2: &'static str = "Hello, world!";


    let hello3:String = hello2.to_string();

}