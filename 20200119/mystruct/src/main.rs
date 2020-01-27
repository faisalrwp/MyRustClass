
#[derive(Debug)]
struct Data
{ name:String,fee:u16,status:bool,}
#[derive(Debug)]
enum pakistan
{ punjab,sindh,kp,balochistan,gb,}

fn main() {
    let student1 = Data { name:String::from("Rehman"),status:true,fee:2000};
    println!("Hello, world! {:#?}",student1);
    let p1=pakistan::punjab;
    println!("Hello, world! {:#?}",p1);
}
