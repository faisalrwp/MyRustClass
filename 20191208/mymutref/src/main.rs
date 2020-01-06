fn main() {
    let mut mystring=String::from("Pakistan");
    println!("{}", mystring);
    mystring.push_str(" Zinda Bad");
    println!("{}", mystring);
    let mystring2 = &mut mystring;
    println!("{}", mystring2);
    mystring2.push_str(" Painda Bad");
    println!("{}", mystring2);

}

