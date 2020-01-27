#[derive(Debug)]

struct Week{
    day1:String,
    day2:String,
}

#[derive(Debug)]
enum Days
{
    sunday(String),
    monday(u8,u8),
    tuesday(i8,f32),
}

#[derive(Debug)]
enum vehicle
{
    Toyota(String,u16),
    Honda(String,u16),
    Quinqhi,
}
fn main() {
    let week1 = Week { day1:String::from("Friday"), day2:String::from("Saturday")};

    let myday1 = Days::sunday(String::from("Holiday"));
    let myday2 = Days::monday(22,33);
    let myday3 = Days::tuesday(22,33.333);

    let myv1 = vehicle::Toyota(String::from("Corolla"),2019);
    let myv2 = vehicle::Honda(String::from("Civic"),2019);
    let myv3 = vehicle::Quinqhi;

    println!("Struct {:#?}",week1);
    println!("Enum {:#?}",myday1);
    println!("Enum {:#?}",myday2);
    println!("Enum {:#?}",myday3);
    println!("Enum {:#?}",myv1);
    println!("Enum {:#?}",myv2);
    println!("Enum {:#?}",myv3);

    test(myday1);
    test(myday2);
    test(myday3);
}

fn test(myday:Days)
{
    match myday
    {
        Days::sunday(a) => println!("TEST {:?}", a),
        Days::monday(a,b) => println!("TEST {} {}", a,b),
        Days::tuesday(a,b) => println!("TEST {} {}", a,b),
    }
}