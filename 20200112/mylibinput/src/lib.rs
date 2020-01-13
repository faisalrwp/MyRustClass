use std::io;

pub fn getString(myText: String) -> String {
    println!("{}", myText);
    let mut txt = String::new();
    io::stdin().read_line(&mut txt).expect("Please Enter Data");
    return txt;
}

pub fn geti32(myText: String) -> i32 {
    println!("{}", myText);
    let mut txt = String::new();
    io::stdin().read_line(&mut txt).expect("Please Enter Data");
    let mynum: i32 = txt.trim().parse().expect("Enter Numeric Data Please");
    return mynum;
}

pub fn getu32(myText: String) -> u32 {
    println!("{}", myText);
    let mut txt = String::new();
    io::stdin().read_line(&mut txt).expect("Please Enter Data");
    let mynum: u32 = txt.trim().parse().expect("Enter Numeric Data Please");
    return mynum;
}

pub fn getf32(myText: String) -> f32 {
    println!("{}", myText);
    let mut txt = String::new();
    io::stdin().read_line(&mut txt).expect("Please Enter Data");
    let mynum: f32 = txt.trim().parse().expect("Enter Numeric Data Please");
    return mynum;
}