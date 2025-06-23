use rand::{rng, seq::SliceRandom};
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

        Deck { cards }
    }

    // doc.rust-lang.org/std
    // crates.io
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let cards = deck.deal(3);
    println!("Heres your hands: {:#?}", cards); // {:?} in single line
    println!("Heres your deck: {:#?}", deck); // {:#?} in multiple easy read line
}
