use rand::Rng;

fn main() {
    println!("Guess The Number");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("the secret number is : {}", secret_number);
}
