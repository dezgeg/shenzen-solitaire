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

fn print_card(card: &Card, is_head: bool) {
    if is_head {
        match *card {
            Card::Dragon(s) => print!("│ {}      │ ", ansi_of_dragon(s)),
            Card::Flower => print!("│  ~~~~  │ "),
            Card::Number(s, n) => print!("│ {}      │ ", style_of_suit(s).paint(n.to_string()), ),
        };
    } else {
        match *card {
            Card::Dragon(s) => print!("│      {} │ ", ansi_of_dragon(s)),
            Card::Flower => print!("│  ~~~~  │ "),
            Card::Number(s, n) => print!("│      {} │ ", style_of_suit(s).paint(n.to_string()), ),
        };
    }
}

// Card drawing: each non-topmost card consists of 1 'head' piece (where 1 piece == 2 lines)
// and the topmost card consists of 4 pieces (head, 2 filler, tail)
//╭────────╮\ head
//│ 2      │/
//╭────────╮\ head
//│ 1      │/
//│        |\ filler 1
//│        │/
//│        │\ filler 2
//│        │/
//│      1 │\ tail
//╰────────╯/
// So for a stack of n cards, we always draw n + 3 pieces.
// Exception: empty stacks are not drawn.

fn print_playfield(playfield: &Playfield) {
    let max_col_height = playfield.tableau.iter().map(|cs| cs.len()).max().unwrap();
    for piece_index in 0..(max_col_height + 3) {
        for line in 1..3 {
            for col in 0..8 {
                let cards_in_column = &playfield.tableau[col];
                let column_height = cards_in_column.len();

                let is_head = piece_index < column_height;
                let is_filler = !is_head && (piece_index < column_height + 2 && !cards_in_column.is_empty());
                let is_tail = piece_index == column_height + 2 && !cards_in_column.is_empty();

                if line == 1 {
                    if is_head {
                        print!("╭────────╮ ");
                    } else if is_filler {
                        print!("│        │ ");
                    } else if is_tail {
                        let card = cards_in_column.get(piece_index - 3).unwrap();
                        print_card(card, false);
                    } else {
                        print!("           ");
                    }
                } else {
                    if is_head {
                        let card = cards_in_column.get(piece_index).unwrap();
                        print_card(card, true);
                    } else if is_filler {
                        print!("│        │ ");
                    } else if is_tail {
                        print!("╰────────╯ ");
                    } else {
                        print!("           ");
                    }
                }
            }
            println!();
        }
    }
}

fn main() {
    let render_test = Playfield {
        freecells: [FreeCell::Free, FreeCell::Flipped, FreeCell::InUse(Card::Dragon(Suit::Black))],
        flower: Some(Card::Flower),
        piles: [None, Some(Card::Number(Suit::Green, 1)), Some(Card::Number(Suit::Black, 9))],
        tableau: [
            /* 0 */ vec![],
            /* 1 */ vec![Card::Number(Suit::Red, 1)],
            /* 2 */ vec![Card::Number(Suit::Red, 1), Card::Number(Suit::Black, 2)],
            /* 3 */ vec![Card::Number(Suit::Red, 1), Card::Number(Suit::Black, 2), Card::Number(Suit::Green, 3)],
            /* 4 */ vec![Card::Number(Suit::Red, 1), Card::Number(Suit::Black, 2), Card::Number(Suit::Green, 3), Card::Number(Suit::Red, 4)],
            /* 5 */ vec![Card::Number(Suit::Red, 5), Card::Number(Suit::Black, 4)],
            /* 6 */ vec![Card::Number(Suit::Red, 6), Card::Number(Suit::Black, 5)],
            /* 7 */ vec![Card::Number(Suit::Red, 1), Card::Number(Suit::Black, 2), Card::Number(Suit::Green, 3), Card::Number(Suit::Red, 4), Card::Number(Suit::Black, 9), Card::Number(Suit::Black, 8), Card::Number(Suit::Black, 7), Card::Number(Suit::Black, 6), Card::Number(Suit::Black, 5), Card::Number(Suit::Black, 4), Card::Number(Suit::Black, 3), Card::Number(Suit::Black, 2), Card::Number(Suit::Black, 1), ],
        ]
    };
    //print_playfield(&make_shuffled_playfield());
    print_playfield(&render_test);
}

