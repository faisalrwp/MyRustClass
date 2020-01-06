fn main() {
    let octal:u8=0o123;
    println!("Decimal {}", octal);
    println!("Binary {:b}", octal);
    println!("octal {:o}", octal);
    println!("Hexa {:x}", octal);

    let byte:u8=b'A';
    println!("Decimal {}", byte);
    println!("Binary {:b}", byte);
    println!("octal {:o}", byte);
    println!("Hexa {:x}", byte);
    //println!("Char {:c}", byte);
}
