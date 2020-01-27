fn main() {
    
    let s1=String::from("hello");
    let s2=s1.clone();
    println!("s1 = {} , s2 = {}",s1,s2);
    println!("Pointer s1 = {:p} , s2 = {:p}",&s1,&s2);

}
