use std::io;
fn main() {
    println!("Enter Your Guess");

    io::stdin().read_line(&mut guess).expect("Failed to READ");
    let ans : u32 = guess.trim().unwrap();

    if ans > 10 {
        panic!("Too Large");
    }
    let data = [11,22,33];
    println!("{:?}", data[ans]);
}
