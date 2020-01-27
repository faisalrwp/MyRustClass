fn main() 
{
    let mut v : Vec<i32> = Vec::new();
    let mut x = vec![11,22,33,44,55];
    for a in 1..=5
    {
        v.push(a);
    }

    println!("{:?} {:?}",v,x);
    for a in &mut v
    {
        * a += 50;
    }
    println!("{:?} {:?}",v,x);
    v[2]=5;
    println!("{:?}",v);
    v.pop();
    println!("{:?}",v);
    v.clear();
    println!("{:?}",v);
    x.push(99);
    println!("{:?} {:?}",v,x);
    let temp = &x[3];
    println!("{:?}",temp);
    let t2 = v.get(100);
    println!("{:?}",t2);
}
