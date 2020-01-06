fn main() {
    let get=giveback();
    println!("We got {}", get);
    let batch3=String::from("IoT");
    let assignment=receiver(batch3);
    println!("Main Function : {:?}", assignment);
    let assignment2=receiver(get);
    println!("Main Function : {:?}", assignment2);
}
fn giveback() -> String
{
    let name=String::from("Batch3");
    name
}
fn receiver(morning:String) -> (String,usize)
{
    println!("Receiver Function : {}", morning);
    let alen  = morning.len();
    (morning, alen)
}

