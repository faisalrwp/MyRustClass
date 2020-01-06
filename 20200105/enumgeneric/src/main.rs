#[derive(Debug)]
enum Options<T>
{
    Some(T),
    None
}

enum MatchTest{value1(u8), value2(u8)}

fn main() {
    let v1=Some(5);
    let v2=Some(3.3);
    let v3=Some(String::from("TEST"));
    println!("{:?} {:?} {:?}", v1,v2,v3);

    let nodata:Option<i32>=None;
    let coa=String::from("Qamar Bajwa");
    println!("Length of {} is {}", coa, coa.len());

    let age = [22,33,44,55,66];
    println!("{}",age[0]);
    let temp=100;
    let data=age.get(temp);
    println!("{:?}",data);

    let m1 = 10u8;
    match m1
    {
        0  => println!("zero"),
        10 => println!("ten"),
        20 => println!("twenty"),
        30 => println!("thirty"),
        _  => println!("other")
    }
}
