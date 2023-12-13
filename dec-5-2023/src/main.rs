use std::fs;
use regex::Regex;

fn main() {
    let file_path = "src/test.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    let file_lines: Vec<_> = contents.lines().collect();

    let first_line: Vec<_> = file_lines[0].split("seeds: ").collect();
    let seeds: Vec<i32> = first_line[1].split(" ").map(|s| s.parse().unwrap()).collect();

    println!("Seeds: {:?}", seeds);

    let re_line = Regex::new(r"\d+ \d+ \d+").unwrap();

    let mut mappings: Vec<Mapping> = Vec::new();

    let mut current_mapping: Mapping = Mapping {
        source: "",
        destination: "",
        lines: Vec::new(),
    };

    // from the third line to the end of the file
    for line in file_lines[2..].iter() {
        if line.contains("map:") {
            current_mapping = Mapping {
                source: "",
                destination: "",
                lines: Vec::new(),
            }
        }

        if line.contains("-to-") {
            let split: Vec<_> = line.split("-to-").collect();
            current_mapping.source = split[0];
            current_mapping.destination = split[1].split(" map:").collect::<Vec<_>>()[0];
        }

        if re_line.is_match(line) {
            current_mapping.lines.push(MappingLine::from_line(line));
        }

        if line == &"" {
            mappings.push(current_mapping.clone());
        }
    }

    for mapping in mappings.iter() {
        println!("Source: {}, Destination: {}", mapping.source, mapping.destination);
        for line in mapping.lines.iter() {
            println!("{:?}", line);
        }
    }
    // split the line by space and store the line in the mapping
}

#[derive(Debug, Clone)]
struct Mapping<'a> {
    source: &'a str,
    destination: &'a str,
    lines: Vec<MappingLine>,
}

#[derive(Debug, Clone)]
struct MappingLine {
    destination_range_start: i32,
    source_range_start: i32,
    range_length: i32,
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