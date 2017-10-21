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
    Number(Suit, usize),
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
    // Stupid boilerplate function
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

// Creates a shuffled, initial state of the game.
// That is, all the 40 cards are evenly shuffled into the 8 tableau columns and the rest is empty.
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

// And finally, rules & logic of the game:

// Available positions on the playfield where cards can be played.
// For the 'usize' indexes, only certain values are legal.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Position {
    // Index must be 0 .. 2 inclusive.
    FreeCell(usize),
    Flower,
    // Index must be 0 .. 2 inclusive.
    Pile(usize),
    // Index must be 0 .. 7 inclusive.
    Tableau(usize),
}

// A move simply moves a number of cards from a position to another position.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Move(usize, Position, Position);

// Returns whether @card1 can be placed on top of @card2 on the tableau.
// That is:
//  - both must be numeric
//  - suits must be different
//  - @card1's value must be one lower than @card2's
pub fn can_place_on_top(card1: Card, card2: Card) -> bool {
    match (card1, card2) {
        (Card::Number(suit1, number1), Card::Number(suit2, number2)) =>
            suit1 != suit2 && number2 == number1 + 1,
        _ => false
    }
}

// Pick up @count cards from the @playfield position @from.
// If this half-move is not permitted by the game rules, None is returned.
// Otherwise, a pair of the following form is returned:
//   - 1st element is the new Playfield object with the lifted card removed
//   - 2nd element is a vector of the picked up cards
//
// Attempting to pick up more cards than a position contains returns None.
pub fn pick_up_cards(playfield: Playfield, count: usize, from: Position) -> Option<(Playfield, Vec<Card>)> {
    assert!(count > 0);
    let mut pf2: Playfield = playfield;
    match from {
        // Cards can't ever be picked up from discard piles or from the flower spot.
        Position::Flower | Position::Pile(_) => return None,
        // Freecells can only have a single card each; additionally flipped-over dragons in free cells
        // can't be messed with.
        Position::FreeCell(fi) => match (pf2.freecells[fi], count) {
            (FreeCell::InUse(old_card), 1) => {
                pf2.freecells[fi] = FreeCell::Free;
                Some((pf2, vec![old_card]))
            }
            _ => None,
        },
        // For a tableau position, the usual can-place-on-top-of rules apply
        // (Card must be numeric, suit must be different and value strictly decreasing by one.)
        Position::Tableau(ti) => {
            let picked_up_cards = {
                let old_cards = &mut pf2.tableau[ti];
                if count > old_cards.len() {
                    return None;
                }
                let idx = old_cards.len() - count;
                old_cards.split_off(idx)
            };
            let mut prev_card = picked_up_cards[0];
            for i in 1..picked_up_cards.len() {
                if can_place_on_top(picked_up_cards[i], prev_card) {
                    prev_card = picked_up_cards[i];
                } else {
                    return None
                }
            }
            Some((pf2, picked_up_cards))
        }
    }
}

// Places the cards in @new_cards onto the position @to on the @playfield.
// If this half-move is not permitted by the game rules, None is returned.
// Otherwise, a new Playfield object with the cards placed appropriately is returned.
//
// Note: This function assumes that @new_cards only comes from the return value of pick_up_cards(),
// otherwise non-rule-conforming behaviour may occur.
pub fn place_cards(playfield: Playfield, new_cards: Vec<Card>, to: Position) -> Option<Playfield> {
    let mut new_pf: Playfield = playfield;
    let bottom_card = new_cards[0];

    // Tableau positions can accept multiple cards, so special-case that first.
    if let Position::Tableau(ti) = to {
        let ok = {
            let old_cards = &mut new_pf.tableau[ti];
            let last = old_cards.last().cloned();
            old_cards.extend(new_cards);
            match (last, bottom_card) {
                // Anything can be moved into empty tableau slots
                (None, _) => true,
                // Otherwise, we just consider if the bottom-most card of @new_cards can be placed
                // on the top card of the tableau pile.
                (Some(Card::Number(dst_suit, dst_number)), Card::Number(src_suit, src_number)) => {
                    dst_number == src_number + 1 && src_suit != dst_suit
                }
                _ => false,
            }
        };
        if ok {
            return Some(new_pf)
        } else {
            return None
        }
    }

    // All the other positions on the board can house only one card at a time.
    if new_cards.len() != 1 {
        return None;
    }

    let ok = match (to, bottom_card) {
        // A free freecell accepts any card, other kinds of freecells don't obviously accept anything.
        (Position::FreeCell(fi), _) => match new_pf.freecells[fi] {
            FreeCell::Free => {
                new_pf.freecells[fi] = FreeCell::InUse(bottom_card);
                true
            }
            _ => false,
        },
        // The flower spot only accepts a flower.
        (Position::Flower, Card::Flower) => {
            new_pf.flower = Some(bottom_card);
            true
        }
        // A pile spot accepts a card of the same suit and a one higher value
        (Position::Pile(pi), Card::Number(src_suit, src_number)) => {
            let pile = &mut new_pf.piles[pi];
            let last = *pile;
            *pile = Some(bottom_card);
            match last {
                Some(Card::Number(dst_suit, dst_number)) => src_suit == dst_suit && src_number == dst_number + 1,
                _ => true,
            }
        }
        _ => false,
    };
    if ok { Some(new_pf) } else { None }
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

pub fn flip_dragon(playfield: Playfield, suit: Suit) -> Option<Playfield> {
    let mut new_pf: Playfield = playfield;

    let mut dst = -1isize;
    let mut count = 0usize;
    for i in 0..new_pf.freecells.len() {
        match new_pf.freecells[i] {
            FreeCell::InUse(Card::Dragon(s)) if s == suit => {
                new_pf.freecells[i] = FreeCell::Free;
                count = count + 1;
                dst = i as isize;
            }
            FreeCell::Free => {
                dst = i as isize;
            }
            _ => {}
        }
    }
    for i in 0..new_pf.tableau.len() {
        match new_pf.tableau[i].last() {
            Some(&Card::Dragon(s)) if s == suit => {
                new_pf.tableau[i].pop();
                count = count + 1;
            }
            _ => (),
        }
    }
    if count != 4 || dst < 0 {
        return None;
    }
    new_pf.freecells[dst as usize] = FreeCell::Flipped;
    Some(new_pf)
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
            /* 6 */ vec![Card::Number(Suit::Black, 6), Card::Number(Suit::Black, 5)],
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
    // Moving two of the same color: not allowed
    assert!(!is_legal_move(&playfield, Move(2, Position::Tableau(6), Position::Tableau(7))));

    /********** Destination checks *********/
    // Move to empty freecell: true
    assert!(is_legal_move(&playfield, Move(1, Position::Tableau(1), Position::FreeCell(0))));
    // Move to flipped freecell: false
    assert!(!is_legal_move(&playfield, Move(1, Position::Tableau(1), Position::FreeCell(1))));
    // Move to in-use freecell: false
    assert!(!is_legal_move(&playfield, Move(1, Position::Tableau(1), Position::FreeCell(2))));

    // Moving (Green 2) on top of (Green 3): Not allowed
    assert!(!is_legal_move(&playfield, Move(1, Position::Tableau(3), Position::Tableau(2))));
    // Moving (Black 2) on top of (Green 3): Allowed
    assert!(is_legal_move(&playfield, Move(1, Position::Tableau(4), Position::Tableau(2))));
    // Moving two cards (Red 4, Green 3) to (Black 5): Allowed
    assert!(is_legal_move(&playfield, Move(2, Position::Tableau(2), Position::Tableau(6))));
}

#[test]
fn test_flip_dragons_on_top_of_each_other() {
    // Can't flip since two dragons are on top of each other
    let pf = Playfield {
        freecells: [FreeCell::Free, FreeCell::Free, FreeCell::InUse(Card::Dragon(Suit::Black))],
        flower: None,
        piles: [None, None, None],
        tableau: [
            /* 0 */ vec![Card::Dragon(Suit::Black), Card::Dragon(Suit::Black)],
            /* 1 */ vec![],
            /* 2 */ vec![Card::Number(Suit::Red, 4), Card::Dragon(Suit::Black)],
            /* 3 */ vec![],
            /* 4 */ vec![],
            /* 5 */ vec![],
            /* 6 */ vec![],
            /* 7 */ vec![],
        ]
    };
    assert!(flip_dragon(pf, Suit::Black) == None);
}

#[test]
fn test_flip_dragons_no_space() {
    // No room in free cells, can't flip
    let pf = Playfield {
        freecells: [FreeCell::InUse(Card::Dragon(Suit::Red)), FreeCell::Flipped, FreeCell::InUse(Card::Dragon(Suit::Red))],
        flower: None,
        piles: [None, None, None],
        tableau: [
            /* 0 */ vec![Card::Dragon(Suit::Black)],
            /* 1 */ vec![Card::Dragon(Suit::Black)],
            /* 2 */ vec![Card::Number(Suit::Red, 4), Card::Dragon(Suit::Black)],
            /* 3 */ vec![Card::Dragon(Suit::Black)],
            /* 4 */ vec![],
            /* 5 */ vec![],
            /* 6 */ vec![],
            /* 7 */ vec![],
        ]
    };
    assert!(flip_dragon(pf, Suit::Black) == None);
}
