use std::collections::HashMap;
fn main() {
    let team = vec!["Pakistan", "Bangladesh"];
    let score = vec![230,220];

    let data:HashMap<_,_> = team.iter().zip(score.iter()).collect();
    println!("{:#?}",data);
    // let mut sc = 0;
    // let mut win = String::from("");


    for (key,value) in &data
    {
        println!("{} : {} ", key, value);
        // if  sc < value
        {
            //sc = value;
            //&win = key;
            println!("test");
        }
            
    }
    // println!("Winner : {} Score : {}", win, sc);
    let mut myteam = HashMap::new();
    myteam.insert("India",20);
    myteam.insert("Pakistan",200);
    myteam.insert("Bangla",120);
    println!("{:#?}",myteam);
    myteam.insert("India",30);
    println!("{:#?}",myteam);

    myteam.entry("India").or_insert(300);
    println!("{:#?}",myteam);

}
