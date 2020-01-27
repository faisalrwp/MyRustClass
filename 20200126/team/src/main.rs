use std::collections::HashMap;


fn main() {
    let mut score = HashMap::new();
    score.insert("Pakistan",330);
    score.insert("Bangladesh",320);

    let pscore = score.get("Pakistan");
    let bscore = score.get("Bangladesh");

    println!("Pakistan : {:#?} \n Bangladesh {:#?}", pscore, bscore);

    
}
