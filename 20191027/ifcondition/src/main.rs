fn main() {
    let result: i8 = 38;
    if result > 90 {
        println!("Test grade is a++");
    } else if result > 80 && result <= 90 {
        println!("Test is grade is a+");
    }
    else if result > 70 && result <= 80 {
        println!("Test is grade is a");
    }
    else if result > 60 && result <= 70 {
        println!("Test is grade is b");
    }
    else if result > 50 && result <= 60 {
        println!("Test is grade is c");
    }
    else
    {
        println!{"You Failed The Test"}
    }
}
