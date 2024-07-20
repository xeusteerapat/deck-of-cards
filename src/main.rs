use rand::{random, seq::SliceRandom, thread_rng};
#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

// inherent methods
impl Deck {
    // it's called a associated function, it's not tied to a specific instance
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Spades", "Clubs"];
        let values = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards } // implicitly return the Deck instance (no need semi-colon)
    }

    // it's called a method, it's tied to a specific instance
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();
    //TODO: do some error handling
    let cards = deck.deal(3);

    println!("Here your deck: {:#?}", deck);
    println!("Here your cards: {:#?}", cards);
}
