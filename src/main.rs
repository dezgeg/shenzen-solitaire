extern crate rand;
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Suit {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Card {
    Number(Suit, u32),
    Dragon(Suit),
    Flower,
}

fn make_deck() -> Vec<Card> {
    let mut ret = Vec::<Card>::new();
    ret.push(Card::Flower);
    for suit in vec![Suit::Red, Suit::Green, Suit::Blue] {
        for number in 1..(9 + 1) {
            ret.push(Card::Number(suit, number));
        }
        ret.push(Card::Dragon(suit));
    }

    ret
}

fn make_shuffled_deck() -> Vec<Card> {
    let mut ret = make_deck();
    rand::thread_rng().shuffle(ret.as_mut_slice());
    ret
}

fn main() {
    println!("deck: {:?}", make_deck());
    println!("shuffled deck: {:?}", make_shuffled_deck());
}
