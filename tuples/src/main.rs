fn main() {
    let _my_info: (&str, i32) = ("Salary", 43);
    println!("{} {}", _my_info.0, _my_info.1);

    let mut arr:[i32; 4] = [3,2,7,45];
    arr[0] = 43;
    arr[1] = 54;
    arr[3] = 111;
    println!("{:?}", arr);

    let mut _arr2: [&str; 3] = ["hello1", "hello1","hello2"];

    println!("{:?}", _arr2);

    let mut number_array:[i32; 4] = [1,2,3,5];

    let subset_array: &[i32] = &number_array[0..3];
    println!("{:?}", subset_array);

    let mut p1: (i32, i32) = (43, 54);
    let mut p2:(i32, i32) = (54,243);

    let mut _res:(f32, f32) = ((p1.0 as f32).abs() - (p2.0 as f32).abs(), (p1.1 as f32).abs() - (p2.1 as f32).abs());
    println!("{:?}", _res);

    let distance: f32 = ((p1.0 as f32 - p2.0 as f32).powf(2.0) + (p1.1 as f32 - p2.1 as f32).powf(2.0)).sqrt();
    println!("{:?}", distance);
}
