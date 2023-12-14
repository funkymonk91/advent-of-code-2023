use std::fs;
use regex::Regex;
use rayon::prelude::*;

fn main() {
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    let file_lines: Vec<_> = contents.lines().collect();

    println!("Part One: Lowest Location {}", part_1(&file_lines));
    println!("Part Two: Lowest Location {}", part_2(&file_lines));
}

fn part_1(file_lines: &Vec<&str>) -> i64 {
    let first_line: Vec<_> = file_lines[0].split("seeds: ").collect();
    let seeds: Vec<i64> = first_line[1].split(" ").map(|s| s.parse().unwrap()).collect();

    let mut lowest_location: Option<i64> = None;

    // println!("Seeds: {:?}", seeds);

    let re_line = Regex::new(r"\d+ \d+ \d+").unwrap();

    let mut mappings: Vec<Mapping> = Vec::new();

    let mut current_mapping: Mapping = Mapping {
        source: "",
        destination: "",
        lines: Vec::new(),
    };

    for (i, line) in file_lines.iter().enumerate() {
        if i == 0 || i == 1 {
            continue;
        }

        if line.contains("-to-") {
            current_mapping = Mapping {
                source: "",
                destination: "",
                lines: Vec::new(),
            };

            let split: Vec<_> = line.split("-to-").collect();
            current_mapping.source = split[0];
            current_mapping.destination = split[1].split(" map:").collect::<Vec<_>>()[0];
        }

        if re_line.is_match(line) {
            current_mapping.lines.push(MappingLine::from_line(line));
        }
        
        // check if the line is empty or is the last line of the file
        if i + 1 == file_lines.len() || line == &"" {
            mappings.push(current_mapping.clone());
        }
    }

    // for mapping in mappings.iter() {
    //     println!("Source: {}, Destination: {}", mapping.source, mapping.destination);

    //     for line in mapping.lines.iter() {
    //         println!("Dest Range Start: {}  Source Range Start: {}  Range Length: {}", line.destination_range_start, line.source_range_start, line.range_length);
    //     }

    //     println!("====");
    // }
    
    for seed in seeds.iter() {
        let mut seed_story: Vec<(String, i64)> = Vec::new();
        let mut source_number = *seed;
        seed_story.push(("seed".to_string(), source_number));

        for mapping in mappings.iter() {
            let mut destination_number = -1;
            for line in mapping.lines.iter() {
                if source_number >= line.source_range_start && source_number <= line.source_range_start + line.range_length {
                    destination_number = line.destination_range_start + (source_number - line.source_range_start);
                    seed_story.push((mapping.destination.to_string(), destination_number));
                    source_number = destination_number;
                    break;
                }
            }
            if destination_number == -1 {
                seed_story.push((mapping.destination.to_string(), source_number));
            }

            if mapping.destination == "location" {
                match lowest_location {
                    Some(lowest) => {
                        if source_number < lowest {
                            lowest_location = Some(source_number);
                        }
                    },
                    None => {
                        lowest_location = Some(source_number);
                    }
                }
            }
        }

        // for story in seed_story.iter() {
        //     print!("{} {}, ", story.0, story.1);
        // }

        // println!("\n\n");
    }

    // println!("Lowest location: {:?}", lowest_location.expect("No lowest location found"));

    return lowest_location.expect("No lowest location found")
}

fn part_2(file_lines: &Vec<&str>) -> i64 {
    let first_line: Vec<_> = file_lines[0].split("seeds: ").collect();
    let re_seeds = Regex::new(r"\d+ \d+").unwrap();

    let seed_ranges: Vec<(i64, i64)> = re_seeds
        .captures_iter(first_line[1])
        .map(|cap| (cap[0].split(" ").collect::<Vec<_>>()[0].parse().unwrap(), cap[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap()))
        .collect();

    let seeds: Vec<_> = seed_ranges.par_iter()
        .flat_map(|range| (range.0 .. range.0 + range.1))
        .collect();

    // println!("Seeds: {:?}", seeds.len());

    let re_line = Regex::new(r"\d+ \d+ \d+").unwrap();

    let mut mappings: Vec<Mapping> = Vec::new();

    let mut current_mapping: Mapping = Mapping {
        source: "",
        destination: "",
        lines: Vec::new(),
    };

    for (i, line) in file_lines.iter().enumerate() {
        if i == 0 || i == 1 {
            continue;
        }

        if line.contains("-to-") {
            current_mapping = Mapping {
                source: "",
                destination: "",
                lines: Vec::new(),
            };

            let split: Vec<_> = line.split("-to-").collect();
            current_mapping.source = split[0];
            current_mapping.destination = split[1].split(" map:").collect::<Vec<_>>()[0];
        }

        if re_line.is_match(line) {
            current_mapping.lines.push(MappingLine::from_line(line));
        }
        
        // check if the line is empty or is the last line of the file
        if i + 1 == file_lines.len() || line == &"" {
            mappings.push(current_mapping.clone());
        }
    }
    
    let lowest_locations: Vec<_> = seeds.par_iter().map(|seed| {
        let mut source_number = *seed;
        let mut seed_lowest_location = None;
    
        for mapping in mappings.iter() {
            let mut destination_number = -1;
            for line in mapping.lines.iter() {
                if source_number >= line.source_range_start && source_number <= line.source_range_start + line.range_length {
                    destination_number = line.destination_range_start + (source_number - line.source_range_start);
                    source_number = destination_number;
                    break;
                }
            }
    
            if mapping.destination == "location" {
                match seed_lowest_location {
                    Some(lowest) => {
                        if source_number < lowest {
                            seed_lowest_location = Some(source_number);
                        }
                    },
                    None => {
                        seed_lowest_location = Some(source_number);
                    }
                }
            }
        }
    
        seed_lowest_location.expect("No lowest location found")
    }).collect();
    
    let lowest_location = *lowest_locations.iter().min().unwrap();

    lowest_location
}

#[derive(Debug, Clone)]
struct Mapping<'a> {
    source: &'a str,
    destination: &'a str,
    lines: Vec<MappingLine>,
}

#[derive(Debug, Clone)]
struct MappingLine {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl MappingLine {
    fn from_line(line: &str) -> MappingLine {
        let split = line.split(" ").collect::<Vec<_>>();
        MappingLine {
            destination_range_start: split[0].parse().unwrap(),
            source_range_start: split[1].parse().unwrap(),
            range_length: split[2].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "src/input.txt";
        let contents = fs::read_to_string(file_path)
            .expect("Something went wrong reading the file");
        let file_lines: Vec<_> = contents.lines().collect();

        let result = part_1(&file_lines); // replace with actual function call and parameters
        assert_eq!(result, 424490994);
    }

    #[test]
    fn test_part_2() {
        let file_path = "src/input.txt";
        let contents = fs::read_to_string(file_path)
            .expect("Something went wrong reading the file");
        let file_lines: Vec<_> = contents.lines().collect();

        let result = part_2(&file_lines); // replace with actual function call and parameters
        assert_eq!(result, 15290096);
    }
}