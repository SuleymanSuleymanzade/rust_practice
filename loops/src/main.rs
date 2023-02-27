fn main() {
    let arr: Vec<i32> = vec![2,3,4,4,34,2];
    for i in arr.iter(){
        println!("{}", i);
    }
    println!();

    for i in 0..arr.len(){
        println!("{}", arr[i]);
    }
    println!();

    let mut arr2: Vec<i32> = vec![2,3,44,3,45,3];

    for i in arr2.iter_mut(){
        *i += 1;
        print!("{} ", i);
    }
    println!();
    print!("{:?}", arr2);
}
