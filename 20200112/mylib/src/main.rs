use std::io;
fn main() {
    println!("Enter A Number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let num:i32=guess.trim().parse().expect("No Number");
    mysl();
    for i in 1..11
    {
        println!("{} x {} = {}", num,i,num*i);
    }
    mysl();
}

fn mysl()
{
    println!("--------------------------");
}