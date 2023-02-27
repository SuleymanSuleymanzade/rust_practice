fn main() {
    let v: Vec<i32> = vec![2,2,3,5,2,6];
    let v2: Vec<i32> = v;
    println!("Hello, world {:?}", v2);

    let mut vv: Vec<i32> = vec![2,2,2,2,2];
    let  reference: &mut Vec<i32> = &mut vv;
    reference[0] = 43;
    print!("{:?}", vv);
}
