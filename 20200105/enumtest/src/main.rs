#[derive(Debug)]
enum option {
    some(u8),
    none,
}

fn main() {
    let data1 = option::some(10);
    let data2:u8=20;
    //let res=d1+d2;
    println!("Hello, world!");
}
