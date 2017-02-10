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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum FreeCell {
    Free,
    InUse(Card),
    // When dragon is placed
    Flipped,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Position {
    FreeCell(u32),
    Flower,
    Pile(u32),
    Tableau(u32),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Move {
    FlipDragon(Suit),
    MoveCards(u32, Position, Position),
}

#[derive(Debug, PartialEq, Eq)]
struct Playfield {
    freecells: [FreeCell; 3],
    // Should only be None or Some(Flower)
    flower: Option<Card>,
    // Topmost card of the pile
    piles: [Option<Card>; 3],
    // Main playfield, works as a stack (so topmost card last in the Vec)
    tableau: [Vec<Card>; 8],
}

fn get_card_at(playfield: Playfield, pos: Position) -> Option<Card> {
    match pos {
        Position::FreeCell(i) => match playfield.freecells[i as usize] {
            FreeCell::InUse(card) => Some(card),
            _ => None,
        },
        Position::Flower => playfield.flower,
        Position::Pile(i) => playfield.piles[i as usize],
        Position::Tableau(i) => playfield.tableau[i as usize].last().map(|x| *x),
    }
}

fn is_legal_move(playfield: Playfield, m: Move) -> bool {
    match m {
        Move::FlipDragon(_) => unreachable!(),
        Move::MoveCards(count, from, to) => {
            unreachable!();
        }
    }
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
