fn main() {
    let value1 = Some(5);
    println!("{:?}", value1);
    let name1=Some(String::from("AR"));
    println!("name : {:?}",name1);
    println!("Hello, world!");
    match value1{
        Some(a) => { let test = a+1; println!("{}",test) },
        None => println!("None"),
    }

    let x = match value1{
        Some(a) => { a+1 },
        None => 0,
    };
    println!("{}",x);
}
