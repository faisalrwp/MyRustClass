use std::collections::HashMap;
use std::io;

fn main() {

    let mut capital = HashMap::new();
    capital.insert(String::from("Pakistan"),String::from("Islamabad"));
    capital.insert(String::from("Punjab"),String::from("Lahore"));
    capital.insert(String::from("Sindh"),String::from("Karachi"));
    capital.insert(String::from("NWFP"),String::from("Peshawar"));
    capital.insert(String::from("Baluchistan"),String::from("Quetta"));
    println!("Hello, world! {:#?}",capital);

    let pk = capital.get("Pakistan");
    println!("{:#?}",pk);

    let pb = capital["Sindh"];
    println!("{:#?}",pb);
    

}
