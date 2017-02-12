mod game_logic;

extern crate ansi_term;

use ansi_term::{ANSIString, Colour, Style};
use game_logic::*;

fn style_of_suit(suit: Suit) -> Style {
    match suit {
        Suit::Red => Style::new().fg(Colour::Red),
        Suit::Green => Style::new().fg(Colour::Green),
        Suit::Black => Style::new().fg(Colour::White),
    }
}

fn ansi_of_dragon(suit: Suit) -> String {
    let c = match suit {
        Suit::Red => "%",
        Suit::Green => "&",
        Suit::Black => "=",
    };
    style_of_suit(suit).paint(c).to_string()
}

fn print_card(card: &Card) {
    match *card {
        Card::Dragon(s) => print!("│ {}      │ ", ansi_of_dragon(s)),
        Card::Flower => print!("│  ~~~~  │ "),
        Card::Number(s, n) => print!("│ {}      │ ", style_of_suit(s).paint(n.to_string()), ),
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
