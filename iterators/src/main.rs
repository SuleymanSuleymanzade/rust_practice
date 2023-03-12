fn main() {
    let a:Vec<u32> = vec![12,3,4,3452,54,3,5421,0,65];
    //let mut check:bool = a.iter().any(|&x| x > 0);

    let filtered_values:Vec<&u32> = a.iter().filter(|&x| *x > 5).collect();
    println!("{:?}", filtered_values);

    let filtered_values_2:Vec<u32> = a.into_iter().filter(|&x| x > 5).collect::<Vec<u32>>();
    println!("{:?}", filtered_values_2);

}
