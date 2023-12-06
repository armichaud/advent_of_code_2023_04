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
        // split around colon
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
        // split the second part around |
        let mut numbers = card.next().unwrap().split("|");
        // convert first item in numbers to list of numbers
        let winning_numbers: Vec<i32> = numbers
            .next()
            .unwrap()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        // convert second item in numbers to list of numbers
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
    assert_eq!(solution_a("example.txt"), 13);
    assert_eq!(solution_a("input.txt"), 21105);
    assert_eq!(solution_b("example.txt"), 30);
    assert_eq!(solution_b("input.txt"), 5329815);
}
