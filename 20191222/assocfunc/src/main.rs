#[derive(Debug)]

struct Food
{
    name:String, price:u16, serving:u8
}

impl Food
{
    fn init(name:String,price:u16,serving:u8) -> Food
    {
        let myfood = Food{name,price,serving};
        return myfood;
    }
    
}

fn main() {
    println!("Hello, world!");
    let fname=String::from("orange");
    let fprice=100;
    let fserving=10;
    let nashta=foodinit(fname,fprice,fserving);
    println!("{:?}", nashta);
    let nashta2=Food::init(String::from("Paratha"),500,1);
    println!("{:?}", nashta2);

}

fn foodinit(name:String,price:u16,serving:u8) -> Food
{
    let myfood = Food{name,price,serving};
    return myfood;
}
