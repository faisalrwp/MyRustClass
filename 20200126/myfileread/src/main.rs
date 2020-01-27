use std::io;
use std::fs;

fn read_username_from_file() -> Result<String,io::Error>
{
    fs::read_to_string("data.txt")
}
fn main() {
    println!("Hello, world!");
    let data = read_username_from_file();
    println!("{:#?}", data);
    
    let x = match data
    {
        some => "T",
        none =>  "E",
    };
    println!("{:#?}", x);

}
