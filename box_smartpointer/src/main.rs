
#[derive(Debug)]
enum List{
    Cons(i32, Option<Box<List>>),
    Nil,
}
use List::{Cons, Nil};

struct MySmartpointer{
    value:i32,
}

impl MySmartpointer{
    fn new(x:i32) -> MySmartpointer{
        MySmartpointer{value:x}
    }
}

use std::ops::Deref;

impl Deref for MySmartpointer{
    type Target = i32;
    fn deref(&self) -> &i32{
        &self.value
    }
}

impl Drop for MySmartpointer{
    fn drop(&mut self){
        println!("dropping my smartpointer from mem {:?}", self.value);
    }
}

fn main() {
    let list:List = List::Cons(23, Some(Box::new(Cons(11, Some(Box::new(Cons(55, None)))))));
    println!("{:?}", list);

    let a = 50;
    let b = &a;
    println!("{}", 50 == a);
    println!("{}", 50 == *b);
    //println!("{}", a == b);
    let sptr1 = MySmartpointer::new(*b);
    let sptr2 = MySmartpointer::new(*b);
    println!("{}", a==*sptr1);

    drop(sptr1);
} 