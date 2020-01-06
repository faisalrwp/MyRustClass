fn main() 
{
    let agei: [i32; 6] = [10, 20, 30, 40, 50, 60];
    let agef: [f32; 6] = [10.5, 20.2, 30.3, 40.4, 50.5, 60.6];

    for value in agei.iter() 
    {
        println!("The value is {}", value);
        println!("The value^2 is {}", value.pow(2));
    }
    println!("==================");
    for value in agef.iter() 
    {
        println!("The value is {}", value);
        println!("The value^2 is {}", value.powi(2));
    }
}
