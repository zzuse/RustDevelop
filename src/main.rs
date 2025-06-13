#[derive(Debug)] //add perticular function
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let deck = Deck { cards: vec![] }; // same as Vec::new()
    println!("Heres your deck: {:?}", deck);
}
