use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {  

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("cannot read the string");

    let secret_number:u32 = rand::thread_rng().gen_range(1..=100);
    let guess:u32 = guess.trim().parse().expect("cannot convert num");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too Hight"),
        Ordering::Equal => println!("Match"),
    }

}
