extern crate rand;
extern crate ansi_term;

use rand::Rng;
use ansi_term::{Colour, Style};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Suit {
    Red,
    Green,
    Black,
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

fn get_card_at(playfield: &Playfield, pos: Position) -> Option<Card> {
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

fn is_legal_move(playfield: &Playfield, m: Move) -> bool {
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

fn make_deck() -> Vec<Card> {
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

fn make_shuffled_deck() -> Vec<Card> {
    let mut ret = make_deck();
    rand::thread_rng().shuffle(ret.as_mut_slice());
    ret
}

fn make_shuffled_playfield() -> Playfield {
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

fn print_card(card: &Card) {
    match *card {
        Card::Dragon(_) => print!("│  !!!!  │ "),
        Card::Flower => print!("│  ~~~~  │ "),
        Card::Number(s, n) => print!("│ {}    {} │ ", n, n, ),
    }
}

fn print_playfield(playfield: &Playfield) {
    for row in 0..5 {
        for col in 0..8 {
            print!("╭────────╮ ");
        }
        println!();
        for col in 0..8 {
            if let Some(card) = playfield.tableau[col].get(row) {
                print_card(card);
            } else {
                print!("     ");
            }
        }
        println!();
    }
    for n in 0..5 {
        for col in 0..8 {
            print!("│        │ ");
        }
        println!();
    }
    for col in 0..8 {
        print!("╰────────╯ ");
    }
    println!();
}

fn main() {
    print_playfield(&make_shuffled_playfield());
}

#[test]
fn test_make_deck() {
    assert_eq!(make_deck().len(), 40);
}

fn make_test_playfield() -> Playfield {
    Playfield {
        freecells: [FreeCell::Free, FreeCell::Flipped, FreeCell::InUse(Card::Dragon(Suit::Black))],
        flower: Some(Card::Flower),
        piles: [None, Some(Card::Number(Suit::Green, 1)), None],
        tableau: [
            /* 0 */ vec![],
            /* 1 */ vec![Card::Dragon(Suit::Red)],
            /* 2 */ vec![Card::Number(Suit::Red, 4), Card::Number(Suit::Green, 3)],
            /* 3 */ vec![Card::Number(Suit::Green, 2)],
            /* 4 */ vec![Card::Number(Suit::Black, 2)],
            /* 5 */ vec![Card::Number(Suit::Black, 4)],
            /* 6 */ vec![],
            /* 7 */ vec![],
        ]
    }
}

#[test]
fn test_get_card_at() {
    let filled = make_test_playfield();
    let empty = Playfield {
        freecells: [FreeCell::Free, FreeCell::Free, FreeCell::Free],
        flower: None,
        piles: [None, None, None],
        tableau: [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]]
    };
    // Free cells
    assert_eq!(get_card_at(&filled, Position::FreeCell(0)), None);
    assert_eq!(get_card_at(&filled, Position::FreeCell(1)), None);
    assert_eq!(get_card_at(&filled, Position::FreeCell(2)), Some(Card::Dragon(Suit::Black)));
    // Flower
    assert_eq!(get_card_at(&filled, Position::FreeCell(2)), Some(Card::Dragon(Suit::Black)));
    assert_eq!(get_card_at(&empty, Position::Flower), None);
    // Piles
    assert_eq!(get_card_at(&filled, Position::Pile(0)), None);
    assert_eq!(get_card_at(&filled, Position::Pile(1)), Some(Card::Number(Suit::Green, 1)));
    // Tableau
    assert_eq!(get_card_at(&filled, Position::Tableau(0)), None);
    assert_eq!(get_card_at(&filled, Position::Tableau(1)), Some(Card::Dragon(Suit::Red)));
    assert_eq!(get_card_at(&filled, Position::Tableau(2)), Some(Card::Number(Suit::Green, 3)));
}

#[test]
fn test_is_legal_move() {
    let playfield = make_test_playfield();

    /********** Source checks *********/
    // Moving the placed flower: not allowed
    assert!(!is_legal_move(&playfield, Move::MoveCards(1, Position::Flower, Position::FreeCell(0))));
    // Moving from the pile: not allowed
    assert!(!is_legal_move(&playfield, Move::MoveCards(1, Position::Pile(1), Position::FreeCell(0))));
    // Moving from a flipped freecell: not allowed
    assert!(!is_legal_move(&playfield, Move::MoveCards(1, Position::FreeCell(1), Position::FreeCell(0))));

    /********** Destination checks *********/
    // Move to empty freecell: true
    assert!(is_legal_move(&playfield, Move::MoveCards(1, Position::Tableau(1), Position::FreeCell(0))));
    // Move to flipped freecell: false
    assert!(!is_legal_move(&playfield, Move::MoveCards(1, Position::Tableau(1), Position::FreeCell(1))));
    // Move to in-use freecell: false
    assert!(!is_legal_move(&playfield, Move::MoveCards(1, Position::Tableau(1), Position::FreeCell(2))));

    // Moving topmost card (Green 3) to (Black 4): Allowed
    assert!(is_legal_move(&playfield, Move::MoveCards(1, Position::Tableau(2), Position::Tableau(5))));
    // Moving (Green 2) on top of (Green 3): Not allowed
    assert!(!is_legal_move(&playfield, Move::MoveCards(1, Position::Tableau(3), Position::Tableau(2))));
    // Moving (Black 2) on top of (Green 3): Allowed
    assert!(is_legal_move(&playfield, Move::MoveCards(1, Position::Tableau(4), Position::Tableau(2))));
}
