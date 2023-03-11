fn main() {
    //convert a Vector of bytes (u8) to a string?
    let bytes = vec![0x41, 0x42, 0x43];
    let s = match String::from_utf8(bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
}
