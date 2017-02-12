extern crate rand;

use self::rand::Rng;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Suit {
    Red,
    Green,
    Black,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Card {
    Number(Suit, u32),
    Dragon(Suit),
    Flower,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FreeCell {
    Free,
    InUse(Card),
    // When dragon is placed
    Flipped,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Position {
    FreeCell(u32),
    Flower,
    Pile(u32),
    Tableau(u32),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Move {
    FlipDragon(Suit),
    MoveCards(u32, Position, Position),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Playfield {
    pub freecells: [FreeCell; 3],
    // Should only be None or Some(Flower)
    pub flower: Option<Card>,
    // Topmost card of the pile
    pub piles: [Option<Card>; 3],
    // Main playfield, works as a stack (so topmost card last in the Vec)
    pub tableau: [Vec<Card>; 8],
}

pub fn get_card_at(playfield: &Playfield, pos: Position) -> Option<Card> {
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

pub fn is_legal_move(playfield: &Playfield, m: Move) -> bool {
    match m {
        Move::FlipDragon(_) => unreachable!(),
        Move::MoveCards(count, from, to) => {
            // Evaluate source card, or bail out if no source card or not allowed to move from source
            let src_card = match from {
                Position::Flower | Position::Pile(_) => return false,
                x => if let Some(card) = get_card_at(playfield, x) { card } else { return false; },
            };
            match to {
                Position::FreeCell(i) => playfield.freecells[i as usize] == FreeCell::Free,
                Position::Flower => src_card == Card::Flower,
                Position::Tableau(_) => {
                    match (src_card, get_card_at(playfield, to)) {
                        (Card::Number(src_suit, src_number), Some(Card::Number(dst_suit, dst_number))) =>
                            return src_number == dst_number - 1 && src_suit != dst_suit,
                        (_, None) => return true,
                        _ => return false,
                    }
                }
                Position::Pile(_) => {
                    match (src_card, get_card_at(playfield, to)) {
                        (Card::Number(src_suit, src_number), Some(Card::Number(dst_suit, dst_number))) =>
                            return src_suit == dst_suit && src_number == dst_number + 1,
                        (Card::Number(_, 1), None) => return true,
                        _ => return false,
                    }
                }
            }
        }
    }
}

pub fn make_deck() -> Vec<Card> {
    let mut ret = Vec::<Card>::new();
    ret.push(Card::Flower);
    for suit in vec![Suit::Red, Suit::Green, Suit::Black] {
        for number in 1..(9 + 1) {
            ret.push(Card::Number(suit, number));
        }
        for i in 0..4 {
            ret.push(Card::Dragon(suit));
        }
    }

    ret
}

pub fn make_shuffled_deck() -> Vec<Card> {
    let mut ret = make_deck();
    rand::thread_rng().shuffle(ret.as_mut_slice());
    ret
}

pub fn make_shuffled_playfield() -> Playfield {
    let mut deck = make_shuffled_deck();
    let mut ret = Playfield {
        freecells: [FreeCell::Free, FreeCell::Free, FreeCell::Free],
        flower: None,
        piles: [None, None, None],
        tableau: [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]]
    };

    for col in 0..8 {
        for row in 0..5 {
            ret.tableau[col].push(deck[8 * row + col]);
        }
    }
    ret
}

