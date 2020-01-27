extern crate short_crypt;
use short_crypt::ShortCrypt;

fn main() {
    println!("Hello, world!");
    let sc = ShortCrypt::new("MagicKey");
    println!("magic key {:#?}", sc);
    let data = sc.encrypt_to_url_component("iot");
    println!("data iot {:#?}", data);

}
