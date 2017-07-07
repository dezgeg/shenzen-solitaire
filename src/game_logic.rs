extern crate rand;

use self::rand::Rng;

// The most basic building blocks - suits & cards:

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

#[test]
fn test_make_deck() {
    assert_eq!(make_deck().len(), 40);
}

// Then the playfield, where the cards are (duh!):

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FreeCell {
    Free,
    InUse(Card),
    // When four dragons are removed from the game and placed onto a free cell
    Flipped,
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

impl Clone for Playfield {
    fn clone(&self) -> Playfield {
        let mut tmp = [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
        for i in 0..tmp.len() {
            tmp[i] = self.tableau[i].clone();
        }
        Playfield {
            freecells: self.freecells.clone(),
            flower: self.flower,
            piles: self.piles.clone(),
            tableau: tmp,
        }
    }
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

// And finally, rules & logic of the game:

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Position {
    FreeCell(u32),
    Flower,
    Pile(u32),
    Tableau(u32),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Move(u32, Position, Position);

pub fn pick_up_cards(playfield: Playfield, count: u32, from: Position) -> Option<(Playfield, Vec<Card>)> {
    let mut pf2 : Playfield = playfield;
    match from {
        Position::Flower | Position::Pile(_) => return None,
        Position::FreeCell(i) => match pf2.freecells[i as usize] {
            FreeCell::InUse(card) => {
                if count != 1 {
                    return None;
                }
                *pf2.freecells.get_mut(i as usize).unwrap() = FreeCell::Free;
                Some((pf2, vec![card]))
            }
            _ => None,
        },
        Position::Tableau(i) => {
            let picked_up_cards = {
                let old_cards = pf2.tableau.get_mut(i as usize).unwrap();
                if count as usize > old_cards.len() {
                    return None;
                }
                old_cards.split_off(count as usize - 1)
            };
            // XXX: check suits
            Some((pf2, picked_up_cards))
        }
    }
}

pub fn place_cards(playfield: Playfield, new_cards: Vec<Card>, to: Position) -> Option<Playfield> {
    let mut pf2 : Playfield = playfield;
    let bottom_card = new_cards[0];
    if let Position::Tableau(ti) = to {
        let ok = {
            let old_cards = &mut pf2.tableau[ti as usize];
            let last = old_cards.last().cloned();
            old_cards.extend(new_cards);
            match (last, bottom_card) {
                (None, _) => true,
                (Some(Card::Number(dst_suit, _)), Card::Number(src_suit, _)) => src_suit != dst_suit, // XXX: check number
                _ => false,
            }
        };
        if ok {
            return Some(pf2)
        } else {
            return None
        }
    }

    if new_cards.len() != 1 {
        return None;
    }

    let ok = match (to, bottom_card) {
        (Position::FreeCell(fi), _) => match pf2.freecells[fi as usize]{
            FreeCell::Free => {
                pf2.freecells[fi as usize] = FreeCell::InUse(bottom_card);
                true
            }
            _ => false,
        },
        (Position::Flower, Card::Flower) => {
            pf2.flower = Some(bottom_card);
            true
        },
        (Position::Pile(pi), Card::Number(src_suit, src_number)) => {
            let pile = &mut pf2.piles[pi as usize];
            let last = *pile;
            *pile = Some(bottom_card);
            match last {
                Some(Card::Number(dst_suit, dst_number)) => src_suit == dst_suit && src_number == dst_number + 1,
                _ => true,
            }
        },
        _ => false,
    };
    if ok { Some(pf2) } else { None }
}

pub fn is_legal_move(playfield: &Playfield, m: Move) -> bool {
    let Move(count, from, to) = m;
    match pick_up_cards((*playfield).clone(), count, from) {
        None => false,
        Some((new_pf, picked_up_cards)) => {
            match place_cards(new_pf, picked_up_cards, to) {
                Some(_) => true,
                None => false,
            }
        }
    }
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
fn test_is_legal_move() {
    let playfield = make_test_playfield();

    /********** Source checks *********/
    // Moving the placed flower: not allowed
    assert!(!is_legal_move(&playfield, Move(1, Position::Flower, Position::FreeCell(0))));
    // Moving from the pile: not allowed
    assert!(!is_legal_move(&playfield, Move(1, Position::Pile(1), Position::FreeCell(0))));
    // Moving from a flipped freecell: not allowed
    assert!(!is_legal_move(&playfield, Move(1, Position::FreeCell(1), Position::FreeCell(0))));

    /********** Destination checks *********/
    // Move to empty freecell: true
    assert!(is_legal_move(&playfield, Move(1, Position::Tableau(1), Position::FreeCell(0))));
    // Move to flipped freecell: false
    assert!(!is_legal_move(&playfield, Move(1, Position::Tableau(1), Position::FreeCell(1))));
    // Move to in-use freecell: false
    assert!(!is_legal_move(&playfield, Move(1, Position::Tableau(1), Position::FreeCell(2))));

    // Moving topmost card (Green 3) to (Black 4): Allowed
    assert!(is_legal_move(&playfield, Move(1, Position::Tableau(2), Position::Tableau(5))));
    // Moving (Green 2) on top of (Green 3): Not allowed
    assert!(!is_legal_move(&playfield, Move(1, Position::Tableau(3), Position::Tableau(2))));
    // Moving (Black 2) on top of (Green 3): Allowed
    assert!(is_legal_move(&playfield, Move(1, Position::Tableau(4), Position::Tableau(2))));
}
