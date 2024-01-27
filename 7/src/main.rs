use std::fs;

fn main() {
    let file_path = "src/input.txt";
    // let file_path = "src/test.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    let file_lines: Vec<_> = contents.lines().collect();

    // println!("Part 1: {}", part_1(&file_lines));
    println!("Part 2: {}", part_2(&file_lines));
}

fn part_1 (file_lines: &Vec<&str>) -> i32 {
    let mut hands: Vec<Hand> = file_lines.iter().map(|line| {
        let line_split: Vec<&str> = line.split(" ").collect();
        let mut cards: Vec<Card> = Vec::new();
        
        for label in line_split[0].chars() {
            let label = label.to_string();
        
            let value = match label.as_str() {
                "A" => 14,
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "T" => 10,
                _ => label.parse::<i32>().unwrap(),
            };
        
            let card = Card {
                label: label, 
                value: value,
            };
        
            cards.push(card);
        }

        let bid = line_split[1].parse::<i32>().unwrap();

        Hand {
            cards: cards.clone(),
            bid: bid,
            highCard: cards.iter().max_by_key(|card| card.value).unwrap().clone(),
        }
    })
    .collect();

    hands.sort_by(|a, b| {
        let a_rank = a.hand_type().ranking();
        let b_rank = b.hand_type().ranking();

        if a_rank == b_rank {
            // compare the first card in each hand
            // if those match, check the second card, etc
            for i in 0..a.cards.len() {
                let a_card = &a.cards[i];
                let b_card = &b.cards[i];
    
                if a_card.value != b_card.value {
                    return a_card.value.cmp(&b_card.value);
                }
            }

            // If all cards have the same value, return Equal
            std::cmp::Ordering::Equal
        }
        else {
            return a.hand_type().ranking().cmp(&b.hand_type().ranking());
        }
    });
    
    let mut total_winnings = 0;

    for (i, hand) in hands.iter().enumerate() {
        hand.cards.iter().for_each(|card| println!("{:?}", card));
        total_winnings += hand.bid * (i as i32 + 1);

        println!("{:?}", hand.hand_type());
        
        println!("Rank: {}, Bid: {}, Winnings: {}", i + 1, hand.bid, hand.bid * (i as i32 + 1));
        println!("---");
    }

    total_winnings
}

fn part_2 (file_lines: &Vec<&str>) -> i32 {
    let mut hands: Vec<Hand> = file_lines.iter().map(|line| {
        let line_split: Vec<&str> = line.split(" ").collect();
        let mut cards: Vec<Card> = Vec::new();
        
        for label in line_split[0].chars() {
            let label = label.to_string();
        
            let value = match label.as_str() {
                "A" => 14,
                "Q" => 12,
                "K" => 13,
                "T" => 10,
                "J" => 1,
                _ => label.parse::<i32>().unwrap(),
            };
        
            let card = Card {
                label: label, 
                value: value,
            };
        
            cards.push(card);
        }

        let bid = line_split[1].parse::<i32>().unwrap();

        Hand {
            cards: cards.clone(),
            bid: bid,
            highCard: cards.iter().max_by_key(|card| card.value).unwrap().clone(),
        }
    })
    .collect();

    hands.sort_by(|a, b| {
        let a_rank = a.hand_type_with_jokers().ranking();
        let b_rank = b.hand_type_with_jokers().ranking();

        if a_rank == b_rank {
            // compare the first card in each hand
            // if those match, check the second card, etc
            for i in 0..a.cards.len() {
                let a_card = &a.cards[i];
                let b_card = &b.cards[i];
    
                if a_card.value != b_card.value {
                    return a_card.value.cmp(&b_card.value);
                }
            }

            // If all cards have the same value, return Equal
            std::cmp::Ordering::Equal
        }
        else {
            return a.hand_type_with_jokers().ranking().cmp(&b.hand_type_with_jokers().ranking());
        }
    });
    
    let mut total_winnings = 0;

    for (i, hand) in hands.iter().enumerate() {
        hand.cards.iter().for_each(|card| println!("{:?}", card));
        total_winnings += hand.bid * (i as i32 + 1);

        println!("Original: {:?}  Wilds: {:?}", hand.hand_type(), hand.hand_type_with_jokers());
        
        println!("Rank: {}, Bid: {}, Winnings: {}", i + 1, hand.bid, hand.bid * (i as i32 + 1));
        println!("---");
    }

    total_winnings
}

#[derive(Debug, Clone)]
struct Card {
    label: String, // changed from &str to String
    value: i32,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: i32,
    highCard: Card,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut hand_type = HandType::HighCard;
        
        for card in &self.cards {
            let mut count = 0;
            for card2 in &self.cards {
                if card.value == card2.value {
                    count += 1;
                }
            }

            if count == 2 {
                hand_type = HandType::Pair;

                for card2 in &self.cards {
                    let mut count2 = 0;

                    if card.value == card2.value {
                        continue;
                    }

                    for card3 in &self.cards {
                        if card2.value == card3.value {
                            count2 += 1;
                        }
                    }

                    if count2 == 2 {
                        hand_type = HandType::TwoPair;
                    }

                    if count2 == 3 {
                        hand_type = HandType::FullHouse;
                    }
                }
            }

            if count == 3 {
                hand_type = HandType::ThreeOfAKind;

                // check for full house
                for card2 in &self.cards {
                    let mut count2 = 0;

                    if card.value == card2.value {
                        continue;
                    }

                    for card3 in &self.cards {
                        if card2.value == card3.value {
                            count2 += 1;
                        }
                    }

                    if count2 == 2 {
                        hand_type = HandType::FullHouse;
                    }
                }
            }

            if count == 4 {
                hand_type = HandType::FourOfAKind;
            }

            if count == 5 {
                hand_type = HandType::FiveOfAKind;
            }
        }

        hand_type
    }

    fn hand_type_with_jokers(&self) -> HandType {
        let mut joker_count = 0;

        for card in &self.cards {
            if card.label == "J" {
                joker_count += 1;
            }
        }

        let mut hand_type = self.hand_type();
        
        match hand_type {
            HandType::HighCard => {
                if &self.highCard.label == "J" && joker_count == 1 {
                    hand_type = HandType::HighCard;
                }
                else {
                    match joker_count {
                        1 => hand_type = HandType::Pair,
                        2 => hand_type = HandType::ThreeOfAKind,
                        3 => hand_type = HandType::FourOfAKind,
                        4 => hand_type = HandType::FiveOfAKind,
                        _ => (),
                    }
                }
            },
            HandType::Pair => {
                match joker_count {
                    1 => hand_type = HandType::ThreeOfAKind,
                    2 => hand_type = HandType::Pair, // The pair is the jokers...
                    3 => hand_type = HandType::FullHouse,
                    4 => hand_type = HandType::FiveOfAKind,
                    5 => hand_type = HandType::FiveOfAKind,
                    _ => (),
                }
            },
            HandType::TwoPair => {
                match joker_count {
                    1 => hand_type = HandType::FullHouse,
                    2 => hand_type = HandType::FourOfAKind,
                    _ => (),
                }
            },
            HandType::ThreeOfAKind => {
                match joker_count {
                    1 => hand_type = HandType::FourOfAKind,
                    2 => hand_type = HandType::FiveOfAKind,
                    3 => hand_type = HandType::ThreeOfAKind,
                    _ => (),
                }
            },
            HandType::FourOfAKind => {
                match joker_count {
                    1 => hand_type = HandType::FiveOfAKind,
                    4 => hand_type = HandType::FourOfAKind,
                    _ => (),
                }
            },
            _ => (),
        }
       
        hand_type
    }
}

#[derive(Debug, PartialEq)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn ranking(&self) -> u8 {
        match self {
            HandType::HighCard => 0,
            HandType::Pair => 1,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 5,
            HandType::FiveOfAKind => 6,
        }
    }
}