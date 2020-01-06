use std::io;

fn main() {
    let mut input1 = String::new();

    while input1.trim() != "END" || input1.trim() != "end" {
        input1 = "".to_string();
        println!("Enter Your Numbers :");
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed In Reading Input");
        println!("You input {:?} ", input1);
        println!("You input {:?} ", input1.trim());
        {
            let mut result: i8 = input1.trim().parse().expect("Failed Conversion");

            println!("=================");
            println!("RESULT GRADES");

            if result > 90 {
                println!("Test grade is a++");
            } else if result > 80 && result <= 90 {
                println!("Test is grade is a+");
            } else if result > 70 && result <= 80 {
                println!("Test is grade is a");
            } else if result > 60 && result <= 70 {
                println!("Test is grade is b");
            } else if result > 50 && result <= 60 {
                println!("Test is grade is c");
            } else {
                println! {"You Failed The Test"}
            }
            println!("=================");
        }
    }
}
