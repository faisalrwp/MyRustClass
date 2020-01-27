use std::fs::File;
fn main() {
    println!("Hello, world!");
    let myf=File::open("Cargo.toml");
    let mynof=File::open("mydata.txt");

    println!("{:#?}", myf);
    println!("{:#?}", mynof);

    let mynof = match mynof
    {
        Ok(mynof) => mynof,
        Err(error) => panic!("The file is not available"), 
    };
}
