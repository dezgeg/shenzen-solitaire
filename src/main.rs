mod game_logic;

extern crate ansi_term;

use ansi_term::{ANSIString, Colour, Style};
use game_logic::*;
use std::io;
use std::io::Write;

fn style_of_suit(suit: Suit) -> Style {
    match suit {
        Suit::Red => Style::new().fg(Colour::Red),
        Suit::Green => Style::new().fg(Colour::Green),
        Suit::Black => Style::new().fg(Colour::Blue),
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

fn print_card_main_part(card: &Card, is_head: bool) -> Vec<String> {
    let mut ret = vec![];
    if is_head {
        ret.push("╭────────╮ ".to_string());
    }
    ret.push(if is_head {
        match *card {
            Card::Dragon(s) => format!("│ {}      │ ", ansi_of_dragon(s)),
            Card::Flower => format!("│  ~~~~  │ "),
            Card::Number(s, n) => format!("│ {}      │ ", style_of_suit(s).paint(n.to_string()), ),
        }
    } else {
        match *card {
            Card::Dragon(s) => format!("│      {} │ ", ansi_of_dragon(s)),
            Card::Flower => format!("│  ~~~~  │ "),
            Card::Number(s, n) => format!("│      {} │ ", style_of_suit(s).paint(n.to_string()), ),
        }
    });
    if !is_head {
        ret.push("╰────────╯ ".to_string());
    }
    ret
}

fn empty_column() -> Vec<String> {
    vec![
        "           ".to_string(),
        "           ".to_string(),
    ]
}

fn filler_column() -> Vec<String> {
    vec![
        "│        │ ".to_string(),
        "│        │ ".to_string(),
    ]
}

fn print_card(card: &Card) -> Vec<String> {
    let mut ret = vec![];
    ret.extend(print_card_main_part(card, true));
    ret.extend(filler_column());
    ret.extend(filler_column());
    ret.extend(print_card_main_part(card, false));
    ret
}

fn print_flipped_card() -> Vec<String> {
    let mut ret = vec![];
    ret.push("╭────────╮ ".to_string());
    for i in 0..6 {
        ret.push("│▒▒▒▒▒▒▒▒│ ".to_string());
    }
    ret.push("╰────────╯ ".to_string());
    ret
}

fn print_free_card() -> Vec<String> {
    let mut ret = vec![];
    ret.push("╭────────╮ ".to_string());
    for i in 0..6 {
        ret.push("│        │ ".to_string());
    }
    ret.push("╰────────╯ ".to_string());
    ret
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

fn print_tableau(playfield: &Playfield) {
    let max_col_height = playfield.tableau.iter().map(|cs| cs.len()).max().unwrap();
    let mut prints: Vec<Vec<String>> = vec![];
    for col in 0..8 {
        let cards_in_column = &playfield.tableau[col];
        let column_height = cards_in_column.len();

        let mut column_lines: Vec<String> = vec![];

        for piece_index in 0..(max_col_height + 3) {
            let is_head = piece_index < column_height;
            let is_filler = !is_head && (piece_index < column_height + 2 && !cards_in_column.is_empty());
            let is_tail = piece_index == column_height + 2 && !cards_in_column.is_empty();

            if is_head {
                column_lines.extend(print_card_main_part(cards_in_column.get(piece_index).unwrap(), true));
            } else if is_filler {
                column_lines.extend(filler_column())
            } else if is_tail {
                column_lines.extend(print_card_main_part(cards_in_column.get(piece_index - 3).unwrap(), false));
            } else {
                column_lines.extend(empty_column());
            }
        }
        prints.push(column_lines);
    }
    for i in 0..prints[0].len() {
        for j in 0..playfield.tableau.len() {
            print!("{}", prints[j][i]);
        }
        println!();
    }
}

fn print_top(playfield: &Playfield) {
    let mut prints: Vec<Vec<String>> = vec![];
    for fc in playfield.freecells.iter() {
        match fc {
            &FreeCell::InUse(c) => {
                prints.push(print_card(&c));
            }
            &FreeCell::Flipped => {
                prints.push(print_flipped_card());
            }
            &FreeCell::Free => {
                prints.push(print_free_card());
            }
        }
    }

    let mut tmp = vec![];
    // Draw flower & dragon symbols here
    for (i, _) in playfield.flipped_suits.iter().enumerate() {
        let style = style_of_suit(Suit::from_index(i)).bold();
        tmp.extend(vec![
            "  ╭─╮   ",
            "  ╰─╯   ",
            "        ",
        ].iter().map(|x| style.paint(x.to_string()).to_string()));
    }
    tmp.pop();
    let tmp2 = vec![
        "              ".to_string(),
        "|        |    ".to_string(),
        "|        |    ".to_string(),
        "|        |    ".to_string(),
        "|        |    ".to_string(),
        "|        |    ".to_string(),
        "|        |    ".to_string(),
        "              ".to_string(),
    ];
    prints.push(tmp);
    prints.push(tmp2);

    for p in playfield.piles.iter() {
        match p {
            &Some(c) => {
                prints.push(print_card(&c));
            }
            _ => prints.push(print_free_card()),
        }
    }

    for i in 0..prints[0].len() {
        for j in 0..prints.len() {
            print!("{}", prints[j][i]);
        }
        println!();
    }
}

fn print_playfield(playfield: &Playfield) {
    print_top(playfield);
    println!();
    print_tableau(playfield);
}

fn test_render() {
    let render_test = Playfield {
        //freecells: [FreeCell::Free, FreeCell::Flipped, FreeCell::InUse(Card::Dragon(Suit::Black))],
        freecells: [FreeCell::InUse(Card::Dragon(Suit::Black)), FreeCell::Flipped, FreeCell::Free],
        flipped_suits: [false, false, false],
        flower: Some(Card::Flower),
        piles: [Some(Card::Number(Suit::Red, 4)), Some(Card::Number(Suit::Green, 1)), Some(Card::Number(Suit::Black, 9))],
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
    print_playfield(&render_test);
}

fn clear() {
    print!("\x1b[H\x1b[2J");
    io::stdout().flush();
}

fn interactive() {
    let playfield = make_shuffled_playfield();

    loop {
        clear();
        print_playfield(&playfield);
        println!();

        print!("Enter move: ");
        io::stdout().flush();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf);
        println!();
    }
}

fn main() {
    interactive();
}
