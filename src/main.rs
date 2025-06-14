#[derive(Debug)] //add perticular function
struct Deck {
    cards: Vec<String>,
}

// inherent implementation
impl Deck {
    // the Self return type equal the parent type
    // the new() is an associated function == class method
    fn new() -> Self {
        let suits = vec!["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"]; // here is the fixed size array, not vector

        let mut cards = vec![]; // Variables are immutable by default
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        let deck = Deck { cards };
        return deck;
    }
}

fn main() {
    let deck = Deck::new();
    println!("Heres your deck: {:#?}", deck); //# easier to read
}
