struct DataStorage<T>{
    DataList: Vec<T>,
    volume: f32,
    title: String,
    mutation_count: u32
}

impl<T: std::ops::Mul + std::ops::MulAssign<<T as std::ops::Mul>::Output>> DataStorage<T>{
    fn new(capacity: usize,
           volume: f32,
           title: String) -> Self{
            
            DataStorage{
                DataList: Vec::with_capacity(capacity),
                volume: volume,
                title: title, 
                mutation_count: 0,
           }
    }
    fn mutate_data(&mut self, salt:T){
            for item in self.DataList.iter(){
                *item *= *item * salt;
            }
            self.mutation_count += 1;
        }
//impl
}

fn main() {
    let arr: Vec<i32> = vec![1,5,32,4,32,4];
    let mut df: DataStorage<i32> = DataStorage::new(10, 0.5, "blockchain".to_string());

    for item in arr.iter(){
        df.DataList.push(*item);
    }

    println!("{:?}", df.DataList);
}
