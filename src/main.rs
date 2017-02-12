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

