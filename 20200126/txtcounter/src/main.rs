use std::collections::HashMap;

fn main() {
    println!("TEXT COUNTER \n");
    let text = " Samjh Samjhna Samjh Kay Samjho Samjh Samjhna Bhee Ek Samjh Hay Samjh Samjh Kay Bhee Jo Na Samjhy Mere Samjh Main Woh Na Samjh Hay";
    let mut mymap = HashMap::new();

    for word in text.split_whitespace()
    {
        let count = mymap.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}",mymap);
}
