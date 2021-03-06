use rand::prelude::ThreadRng;
use rand::Rng;
use std::any::Any;
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use std::{fmt, io};

enum Card {
    King,
    Queen,
    Ace,
    Jack,
    _10,
    _9,
    _8,
    _7,
    _6,
}

impl Card {
    fn get_points(&self) -> u32 {
        match self {
            Card::Jack => 2,
            Card::Queen => 3,
            Card::King => 4,
            Card::_6 => 6,
            Card::_7 => 7,
            Card::_8 => 8,
            Card::_9 => 9,
            Card::_10 => 10,
            Card::Ace => 11,
        }
    }
}

impl std::cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Card::King => {
                    "Король"
                }
                Card::Queen => {
                    "Королева"
                }
                Card::Jack => {
                    "Валет"
                }
                Card::Ace => {
                    "Туз"
                }
                Card::_6 => {
                    "Шестёрка"
                }
                Card::_7 => {
                    "Семёрка"
                }
                Card::_8 => {
                    "Восьмёрка"
                }
                Card::_9 => {
                    "Девятка"
                }
                Card::_10 => {
                    "Десятка"
                }
            }
        )
    }
}

struct Deck(Vec<Card>);

impl Deck {
    fn new() -> Deck {
        Deck(vec![
            Card::King,
            Card::King,
            Card::King,
            Card::King,
            Card::Queen,
            Card::Queen,
            Card::Queen,
            Card::Queen,
            Card::Ace,
            Card::Ace,
            Card::Ace,
            Card::Ace,
            Card::Jack,
            Card::Jack,
            Card::Jack,
            Card::Jack,
            Card::_10,
            Card::_10,
            Card::_10,
            Card::_10,
            Card::_9,
            Card::_9,
            Card::_9,
            Card::_9,
            Card::_8,
            Card::_8,
            Card::_8,
            Card::_8,
            Card::_7,
            Card::_7,
            Card::_7,
            Card::_7,
            Card::_6,
            Card::_6,
            Card::_6,
            Card::_6,
        ])
    }
    fn get_random_card(&mut self) -> Card {
        let index = rand::thread_rng().gen_range(0, self.0.len());
        self.0.remove(index)
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[ ");
        for d in self.0.iter() {
            write!(f, "{}", d);
            if match self.0.last() {
                Some(res) => res != d,
                _ => false,
            } {
                write!(f, ", ");
            }
        }
        write!(f, " ]");
        Ok(())
    }
}

struct Player {
    id: u32,
    cards: Deck,
}

impl Player {
    fn new(id: u32) -> Player {
        Player {
            id,
            cards: Deck(vec![]),
        }
    }
    fn give_card(&mut self, card: Card) {
        self.cards.0.push(card);
    }
    fn get_score(&self) -> u32 {
        let mut sum = 0;
        for c in self.cards.0.iter() {
            sum += c.get_points()
        }
        sum
    }
}
impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "№{}, очки: {}", self.id, self.get_score());
        Ok(())
    }
}
fn main() {
    let p_count: u32 = loop {
        println!("Введите колличество игроков: ");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf);
        match buf.trim().parse::<u32>() {
            Ok(a) => {
                break a;
            }
            Err(a) => println!("Не верное число"),
        }
    };

    let mut deck = Deck::new();
    let mut players: Vec<Player> = vec![];
    let mut i = 0;
    while i < p_count {
        players.push(Player::new(i));
        i += 1;
    }
    for p in (players).iter_mut() {
        println!("Игрок {}", p);
        p.give_card(deck.get_random_card());
        println!("{}", (&p.cards));
        while p.get_score() < 21 {
            println!("Взять карту? (Y/n)");
            let mut answer: String = String::new();
            io::stdin().read_line(&mut answer);
            answer = answer.trim().to_lowercase();
            match &*answer {
                "y" => {
                    p.give_card(deck.get_random_card());
                    println!("{}", (&p.cards));
                }
                "n" => break,
                _ => continue,
            }
        }
        println!("Игрок {}", p);
    }

    println!("Вся колода: {}", &deck);
    players
        .iter()
        .filter(|p| p.get_score() <= 21)
        .max_by(|a, b| {
            if a.get_score() > b.get_score() {
                Ordering::Greater
            } else if a.get_score() < b.get_score() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .and_then(|v| {
            print!("Победил: {}", v);
            Option::Some(v)
        });
}
