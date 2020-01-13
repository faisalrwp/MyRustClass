use myinfo;
use mylibcalc;
use std::io;
mod lib;

fn main() {
    myinfo::myinfo();
    let mut myChoice = 0;
    loop 
    {
        myChoice = mylibinput::getu32(String::from(
            "Enter Option \n 1: Table \n 2:Calculator \n 0: Exit ",
        ));
        myinfo::mysl();
        if myChoice == 1 
        {
            println!("Enter A Number");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            let num: i32 = guess.trim().parse().expect("No Number");
            myinfo::mysl();
            // for i in 1..11
            // {
            //     println!("{} x {} = {}", num,i,num*i);
            // }
            lib::mytable(num);
            myinfo::mysl();
        } 
        else if myChoice == 2 
        {

            let num1 = mylibinput::geti32(String::from("Enter First Number : "));
            let num2 = mylibinput::geti32(String::from("Enter Second Number : "));
            myinfo::mysl();
            let myoperation=mylibinput::geti32(String::from("Select Operation \n 1-Add \n 2-Subtract \n 3-Multiply \n 4-Divide \n 5-Reminder \n Enter Choice : "));
            myinfo::mysl();
            let mut myres = 0;
            match myoperation 
            {
                1 => myres = mylibcalc::add(num1, num2),
                2 => myres = mylibcalc::sub(num1, num2),
                3 => myres = mylibcalc::mul(num1, num2),
                4 => myres = mylibcalc::div(num1, num2),
                5 => myres = mylibcalc::rem(num1, num2),
                _ => myres = 0,
            }
            println!("Result is {}", myres);
            myinfo::mysl();
        } 
        else if myChoice == 0 
        {
            break;
        } 
        else 
        {
            myinfo::mysl();
            println!("Enter 1 or 2 only");
            myinfo::mysl();
        }
    }
}
