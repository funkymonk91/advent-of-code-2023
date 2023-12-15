use std::fs;
use regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
struct ScratchCard {
    number: i32,
    winning_numbers: Vec<i32>,
    card_numbers: Vec<i32>,
    matched_numbers: Vec<i32>,
    points: i32,
}

fn main() {
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    let file_lines: Vec<_> = contents.lines().collect();

    let mut scratch_cards: Vec<ScratchCard> = Vec::new();
    let mut pointTotal = 0;

    let re_number = Regex::new(r"\d+").unwrap();

    for (i, line) in file_lines.iter().enumerate() {
        let numbers = line.split(":").collect::<Vec<&str>>()[1].trim();
        let card_number = re_number.find(line).unwrap().as_str();

        let splits = numbers.split("|").collect::<Vec<&str>>();
        let winning_numbers = splits[0].split(" ").filter(|s| !s.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let card_numbers = splits[1].split(" ").filter(|s| !s.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let matched_numbers: Vec<i32> = winning_numbers.iter().filter(|&n| card_numbers.contains(n)).cloned().collect();

        let mut points = 0;

        // The first match makes the card worth one point and each match after the first doubles the point value of that card.
        for (i, matched_number) in matched_numbers.iter().enumerate() {
            if i == 0 {
                points += 1;
            } else {
                points = 2 * points;
            }
        }

        let scratch_card = ScratchCard {
            number: card_number.parse::<i32>().unwrap(),
            winning_numbers: winning_numbers,
            card_numbers: card_numbers,
            matched_numbers: matched_numbers,
            points: points,
        };

        // println!("{:?}", scratch_card);

        scratch_cards.push(scratch_card);
        pointTotal += points;
    }
    // println!("Total points: {}", pointTotal);

    // Part 2

    let matches_per_card: Vec<usize> = scratch_cards
        .iter()
        .map(|card| card.matched_numbers.len())
        .collect();
    let mut card_counts = vec![1; matches_per_card.len()];

    let mut total_cards = matches_per_card.len();
    loop {
        let mut cards_won = 0;
        for (i, num_matches) in matches_per_card.iter().enumerate() {
            if card_counts[i] == 0 {
                continue;
            }
            card_counts[i] -= 1;
            for j in i + 1..=i + num_matches {
                card_counts[j] += 1;
                cards_won += 1;
                total_cards += 1;
            }
        }
        if cards_won == 0 {
            break;
        }
    }

    // total_cards
    println!("Total cards: {}", total_cards);

}
