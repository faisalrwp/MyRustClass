#[derive(Debug)]

struct IotStudent
{
    name:String, s1:u8,s2:u8,s3:u8
}

#[derive(Debug)]
struct programmer
{
    name:IotStudent,price:u32
}

fn main() 
{
    let st1 = IotStudent{name:String::from("Talemand"),s1:88,s2:99,s3:77};
    let mut st2 = IotStudent{name:String::from("Muneer"),s1:66,s2:67,s3:68};
    let st3 = IotStudent{name:st1.name.clone(),s1:st2.s1,s2:st2.s2,s3:st2.s3};
    let st4 = IotStudent{name:String::from("Sana"),..st2 };
    let pro1 = programmer{name:{name:String::from("a"),s1:1,s2:1,s3:1},price:100};
    println!("{} - {} - {} - {}",st1.name,st1.s1,st1.s2,st1.s3);
    println!("{:?}",st1);
    println!("{:?}",st2);
    st2.s2 = 99;
    println!("{:?}",st2);
    println!("{:?}",st3);
    println!("{:?}",st4);
    println!("{:?}",pro1);

}
