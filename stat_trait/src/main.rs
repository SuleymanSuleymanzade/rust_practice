
struct DataList{
    data_title: String,
    data: Vec<f32>
}

trait StatData{
    fn get_mean(&self) -> f32;
    fn get_variance(&self) -> f32;
    fn get_standard_deviation(&self) -> f32;
    fn set_title(&mut self, title: String);
    fn get_title(&self) -> &String;
}

impl StatData for DataList{
    fn get_mean(&self) -> f32{
        let mut sum: f32 = 0.;
        for item in self.data.iter(){
            sum += *item;
        }
        sum / self.data.len() as f32
    }

    fn get_variance(&self) -> f32{
        let mut acc: f32 = 0.;
        for item in self.data.iter(){
            acc += (*item - self.get_mean()) * (*item - self.get_mean());
        }
        acc / self.data.len() as f32
    }
    
    fn get_standard_deviation(&self) -> f32{
        self.get_variance().sqrt()
    }

    fn set_title(&mut self, title:String){
        self.data_title = title;
    }

    fn get_title(&self) -> &String{
        &self.data_title
    }
}

fn main() {
    let mut my_data = DataList{
        data_title: String::from("list"),
        data: vec![2.,43.,2.,4.,32.,3.,4.,32.,4.,32.,4.,2.,54.,6.7,4.7] 
    };

    my_data.set_title("stat list".to_string());
    println!("{}", my_data.get_title());

    println!("the mean is {}", my_data.get_mean());
    println!("the mean is {}", my_data.get_variance());
    println!("the mean is {}", my_data.get_standard_deviation());


}
