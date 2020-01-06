fn main() {

    let ans = calc(6,3);
    println!("{:?}",ans);
}

fn calc(a:u32,b:u32) -> (u32,u32,u32,u32)
{
    //println!("{}",a);
    //println!("{}",b);
    //println!("{}",a+b);
    (a+b, a-b,a*b,a/b)
}
