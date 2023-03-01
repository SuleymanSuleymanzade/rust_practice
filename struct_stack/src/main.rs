struct Stack{
    capacity: usize,
    mem: Vec<i32>,
}

impl Stack{

    fn new(capacity: usize)->Self{
        Self{
            capacity: capacity, 
            mem: Vec::with_capacity(capacity)
        }
        
    }

    fn push(&mut self, item:i32){
        if self.mem.len() < self.capacity{
            self.mem.push(item);
        }
    }

    fn pop(&mut self) -> Option<i32>{
        let item: Option<i32> = self.mem.pop();
        item
    }

    fn update_capacity(&mut self, new_capacity: usize){
        if new_capacity > self.capacity{
            self.capacity = new_capacity;
        }
    }
}


fn main() {

    let mut stack = Stack::new(32);

    stack.push(342);
    stack.push(432);
    stack.push(543543);
    println!("---------------------------");
    println!("{:?}", stack.mem);

    stack.mem[0] = 432;

}
