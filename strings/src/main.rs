/*
-- string
    - &str
    - String
*/

fn main() {

    println!("Hello, world!");

    let some_string: &str = "fixed length string";
    let mut growable_string: String = String::from("this is string column");
    println!("{}", growable_string);

    growable_string.push('s');
    println!("{}", growable_string);
    growable_string.pop();
    println!("{}", growable_string);

    growable_string.push_str("hello this is additional");
    println!("{}",growable_string);
    let _x:i32 = 5;
    let a: String = _x.to_string();
    println!("{}", a);

    let concated_string:String = format!("{}{}", "first", "second");
    println!("{}",concated_string);
}
