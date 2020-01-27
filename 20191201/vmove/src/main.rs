fn main() {
    let x=5;
    let y=x;
    println!("x={} , y={}", x,y);


    let s1=String::from("Hello");
    println!("s1={:p}",&s1);

    let s2 = s1;
    
    println!("s2={}",s2);
    println!("s2={:p}",&s2);

}
