// comment
fn main() 
{
    {
        let age: u8 = 55;
        println!("Age is {}", age);
        println!("Pointer Of Age is {:p}", &age);
    }
    {
        let mut student_name = String::from("A b C d");
        println!("Name is {}", student_name);
        println!("Name's Length is {}", student_name.len());
        println!("Pointer Of Name is {:p}", &student_name);
        student_name.push_str(" E f");
        println!("Name is {}", student_name);
        println!("Name's Length is {}", student_name.len());
        student_name.push_str(&123.to_string());
        println!("Name is {}", student_name);


    }
    {
        let string_data="Another Way".to_string();
        let simple_data="Its String Literal";
        let string_vardata=simple_data.to_string();

    }
}
