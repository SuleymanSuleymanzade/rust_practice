#[derive(Debug)]
enum List{
    Cons(i32, Option<Box<List>>),
    Nil,
}
use List::{Cons, Nil};

#[derive(Debug)]
enum UpdatedList{

}

struct MySmartpointer{
    value:i32,
}

impl MySmartpointer{
    fn new(x:i32) -> MySmartpointer{
        MySmartpointer{value:x}
    }
}

fn main() {
    let list:List = List::Cons(23, Some(Box::new(Cons(11, Some(Box::new(Cons(55, None)))))));
    println!("{:?}", list);

    let a = 50;
    let b = &a;
    println!("{}", 50 == a);
    println!("{}", 50 == *b);
    println!("{}", a == *b);

} 