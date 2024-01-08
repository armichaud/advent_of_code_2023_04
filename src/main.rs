use std::{fs::File, io::Read};

#[derive(Debug)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    lottery_numbers: Vec<i32>,
    copies: i32,
}

impl Card {
    fn new(id: i32, winning_numbers: Vec<i32>, lottery_numbers: Vec<i32>) -> Card {
        Card {
            id,
            winning_numbers,
            lottery_numbers,
            copies: 1,
        }
    }
}

fn get_cards(path: &str ) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().for_each(|line| {
        let mut card = line.split(":");
        let card_id = card
            .next()
            .unwrap()
            .split(" ")
            .filter(|&x| 
                !x.is_empty() && x.chars().all(|c| c.is_numeric())
            ).next()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mut numbers = card.next().unwrap().split("|");
        let winning_numbers: Vec<i32> = numbers
            .next()
            .unwrap()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let lottery_numbers: Vec<i32> = numbers
            .next()
            .unwrap()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        cards.push(Card::new(card_id, winning_numbers, lottery_numbers));
    });
    cards
}

fn get_matches(cards: &Vec<Card>, i: usize) -> usize {
    cards[i].winning_numbers
        .iter()
        .filter(|&x| cards[i].lottery_numbers.contains(x))
        .count()
}

fn solution_a(path: &str) -> i32 {
    let mut sum = 0;
    let cards = get_cards(path);
     for i in 0..cards.len() {
        let matches = get_matches(&cards, i);
        if matches > 0 {
            sum += 2_i32.pow(matches as u32 - 1);
        }
    }
    sum
}

fn solution_b(path: &str) -> i32{
    let mut cards = get_cards(path);
    for i in 0..cards.len() {
        let matches = get_matches(&cards, i);
        for index in cards[i].id..cards[i].id + matches as i32 {
            cards[index as usize].copies += cards[i].copies;
        }
    }
    cards.iter().fold(0, |acc, card| acc + card.copies)
}

fn main() {
    println!("{}", solution_a("example.txt"));
    println!("{}", solution_a("input.txt"));
    println!("{}", solution_b("example.txt"));
    println!("{}", solution_b("input.txt"));
}
