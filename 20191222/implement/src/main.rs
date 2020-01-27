#[derive(Debug)]
struct student
{ name:String, Age:u8, Batch:u8}

fn main() {
    println!("Hello, world!");
    let st1 = student { name:"Waseem".to_string(), Age:22,Batch:3};
    let st2 = student { name:"Naseem".to_string(), Age:32,Batch:3};

    st1.printer();
    st2.printer();
    let age = st2.getAge();
    let batch = st2.getBatch();
    println!("Age Is {} & Batch is {}",age, batch);
    // let oldername= st2.getOlder(&st1).to_string();
    // println!("Older Is {}", oldername);    
    printer(st2);

}

impl student
{
    fn printer(&self)
    { println!("{:?}",&self); }    
    
    fn getAge(&self) -> u8
    { self.Age }
    
    fn getBatch(&self) -> u8
    { self.Batch }

    fn getOlder(&self,other:&student) -> String
    { let mut name="".to_string();
        if &self.Age > &other.Age
        { name= &self.name.to_string(); }
        else
        { name = &other.name.to_string(); }
        return name
    }
}

fn printer(s:student)
{ println!("{:?}",s); }