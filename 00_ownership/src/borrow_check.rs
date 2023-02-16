fn main() {
    let r;
    {
        let x = 5;
        //r = &x;  //提示   ^^ borrowed value does not live long enough  `x` dropped here while still borrowed
        r = 1;
    }

    println!("r: {}", r);
}
