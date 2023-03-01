
fn create_stack(n: usize) ->  Vec<i32>{
    let mut vec = Vec::with_capacity(n);
    vec
}

fn push(item: i32, vec: &mut Vec<i32>){
    vec.push(item);
}

fn pop(vec: &mut Vec<i32>) -> Option<i32>{
    let num: Option<i32> = vec.pop();
    num
}

fn input()->i32{
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).expect("cannot read file");

    let n: i32 = s.trim().parse().expect("wrong input");
    n

}

fn main() {
    let a:i32 = input();
    let b:i32 = input();
    
    

}
