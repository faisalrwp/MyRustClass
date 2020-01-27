#[derive(Debug)]
struct person{ name:String,age:u8}

fn main() {
    println!("Hello, world!");
    let st1 = (String::from("ali"), 100,200);
    printer(st1);
    let p1=person{name:String::from("person1"),age:100};
    let p2 = stprinter(p1);
    println!{"{:?}",p2};

}
fn stprinter(myst:person) -> person
{
    println!{"{:?}",myst};
    myst
}
fn printer(mytup:(String,i32,i32))
{
    println!("{:?}", mytup);
}
