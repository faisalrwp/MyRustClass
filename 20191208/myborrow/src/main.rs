fn main() {
    let player1=String::from("Shahid Afridi");
    let mylen=calclen(&player1);
    println!("Data : {}", player1);
    println!("Pointer : {:p}", &player1);
    println!("Length : {}", mylen);
    
}

fn calclen(mystr:&String) -> usize
{
    mystr.len()
}
