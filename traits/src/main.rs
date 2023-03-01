
struct Person{
    name: String,
    salary: u32,
    gender: char,
    x_position: f32,
    y_position: f32
}

struct Animal{
    title: String,
    a_type: String,
    gender: char,
    x_position: f32,
    y_position: f32
}

trait Activity{
    fn change_position(&mut self, vertical_step:f32, horisontal_step:f32);
    fn get_position(&mut self) -> (f32, f32);
    fn get_name(&mut self) -> &String;
}


impl Activity for Person{
    fn change_position(&mut self, vertical_step:f32, horisontal_step:f32){
        self.x_position += horisontal_step;
        self.y_position += vertical_step;
    }
    fn get_position(&mut self) -> (f32, f32){
        (self.x_position, self.y_position)
    }
    fn get_name(&mut self) -> &String{
        &self.name 
    }
}

impl Activity for Animal{
    fn change_position(&mut self, vertical_step:f32, horisontal_step: f32){
        self.x_position += horisontal_step;
        self.y_position += vertical_step;
    }
    fn get_position(&mut self) -> (f32, f32){
        (self.x_position, self.y_position)
    }
    fn get_name(&mut self) -> &String{
        &self.title
    }
}

fn main() {
    let mut person = Person{
        name: String::from("Suleyman"),
        salary: 123456,
        gender: 'M',
        x_position: 0.,
        y_position: 0.
    };

    let mut animal = Animal{
        title: String::from("Friend"),
        a_type: String::from("Dog"),
        gender: 'M',
        x_position: 0.,
        y_position: 0.
    };

    animal.change_position(12., 54.);
    animal.change_position(23., 54.);
    let (a, b) = animal.get_position();
    println!("-----------------------------");
    println!("{} {}", a, b);
}
