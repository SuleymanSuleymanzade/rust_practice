fn main() {
    println!("Hello, world!");
    add_name("name1", "sname2", 43);
    add_name("name3", "sname43", 3);
    add_name("name5", "sname45", 22);
    add_name("name6", "sname432", 33);

    let res:i32 = multiply(34, 23);
    println!("{:?}", res);

    let res2:(f32, f32) = get_all_procedures(33., 55.);
    println!("{:?}", res2);

    //let (a1:f32, b1:f32) = get_all_procedures(212., 43.);
    //println!("{} {}", a1, b1 );

    let mut n: String = String::new();
    std::io::stdin().read_line(&mut n).expect("failed during reading");

    let mut num:i32 = n.trim().parse().expect("wrong convertion");
    println!("{}", n);



}

fn add_name(name: &str, fname: &str, age: i32){
    println!("{} {} {}", name, fname, age);
}

fn multiply(num1: i32, num2: i32) -> i32{
    let res:i32 = num1 * num2;
    return res;
}

fn get_all_procedures(num1: f32, num2: f32) -> (f32, f32){
    let res1: f32 =  num1 + num2; 
    let res2: f32 = num1 * num2;
    return (res1, res2);
}