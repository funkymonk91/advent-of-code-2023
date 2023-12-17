use std::fs;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_1(&contents));
}

fn part_1 (input: &str) -> i32 {
    let re_line = Regex::new(r"\d+").unwrap();

    let (times, distances_to_beat) = input
        .split('\n')
        .map(|line| {
            re_line
                .find_iter(line.split_once(':').unwrap().1)
                .map(|m| m.as_str().parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect_tuple()
        .unwrap();


    let mut race_data: Vec<Race> = Vec::new();

    for (time, goal) in times.iter().zip(distances_to_beat.iter()) {
        println!("Time: {} Goal: {}", time, goal);

        let winning_hold_ms_count = (0..*time)
            .enumerate()
            .filter(|(hold_ms, _)| {
                let distance = calc_distance(&time, &hold_ms);

                if distance > *goal {
                    // println!("Hold: {}", hold_ms);
                    return true;
                }

                false
            })
            .collect::<Vec<(usize, i32)>>()
            .len();

        race_data.push(Race {
            time: *time,
            distance: *goal,
            margin: winning_hold_ms_count as i32,
        });

        // println!("Winning hold ms count: {:?}", winning_hold_ms_count);
    }

    let mut answer = 1;
    race_data.iter().for_each(| race | answer *= race.margin);
   
    answer
}

fn part_2 (input: &str) -> i64 {
    let re_line = Regex::new(r"\d+").unwrap();
    
    // get the first line of the file, split on : and then use the regex to get the number i do not want arrays
    let (times, distances_to_beat) = input
        .split('\n')
        .map(|line| {
            re_line
                .find_iter(line.split_once(':').unwrap().1)
                .map(|m| m.as_str().parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect_tuple()
        .unwrap();

    let time: i64 = times[0];
    let goal: i64 = distances_to_beat[0];


     let ways_to_beat_record = (0..time)
        .enumerate()
        .filter(|(hold_ms, _)| {
            let distance = calc_distance(&time, &hold_ms);

            if distance > goal {
                // println!("Hold: {}", hold_ms);
                return true;
            }

            false
        })
        .collect::<Vec<(usize, i64)>>()
        .len();

    ways_to_beat_record as i64
}

fn calc_distance (time: &i64, hold_ms: &usize) -> i64 {
    *hold_ms as i64 * (time - *hold_ms as i64)
}