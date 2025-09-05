use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // List of suits
        let suits = ["Hearts", "Diamonds", "Spades", "Clubs"];

        // List of values
        let values = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];
        // Double nested for loop

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

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

    let hand_one = deck.deal(4);
    let hand_two = deck.deal(4);
    println!("Heres your deck: {:#?}", deck);
    println!("Hand 1: {:#?}", hand_one);
    println!("Hand 2: {:#?}", hand_two);
}
