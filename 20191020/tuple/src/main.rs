fn main() {
    let data = ("piaic", 3000, 67.5);
    let (org, fee, percent) = data;
    println!("The fee is {}", fee);
    let org1 = data.0;
    println!("The organization is : {}", org1);
    println!("Complete tuple : {:?}", data);
}
