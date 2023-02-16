fn update_array1(mut v: [i32; 3]) -> [i32; 3] {
    v[0] = 3;
    assert_eq!([3, 2, 3], v);
    v
}

fn update_array2(v: &mut [i32; 3]) {
    v[0] = 3;
}

fn main() {
    let v1 = [1, 2, 3];
    update_array1(v1);
    assert_eq!([1, 2, 3], v1);

    let mut v2 = [1, 2, 3];
    update_array2(&mut v2);
}
