fn main() {
    println!("Hello, world!");
    let mut v: Vec<i32> = vec![54,54,352,43,54,325,23];
    println!("{:?}", v);
    v[0] = 43;
    v[1] = 534;
    v[3] =1211;
    println!("{:?}", v);
    v.push(43);
    v.push(555);
    println!("{:?}", v);

    let v2: Vec<&str> = vec!["Unknown"; 3];
    println!("{:?}", v2);

    let mut graph: Vec<Vec<i32>> = vec![vec![3;4]];
    println!("{:?}", graph);
    graph[0].push(43);
    graph[0].push(555);
    println!("{:?}", graph);
}
