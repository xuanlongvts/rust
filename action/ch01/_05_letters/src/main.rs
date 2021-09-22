fn main() {
    let mut letters = vec!["a", "b", "c"];

    for item in letters {
        println!("item: {}", item);
        
        letters.push(item.clone());
    }
    // letters.push("d");
}
