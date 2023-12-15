use std::fs;
use fancy_regex::Regex; // need for lookaheads

fn main() {
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let re = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    let mut numbers: Vec<Vec<u32>> = Vec::new();
    let mut total: u32 = 0;
    let file_lines: Vec<_> = contents.lines().collect();
    
    for line in &file_lines {
        // Initialize a vector to hold the numbers found in this line
        let mut line_numbers: Vec<u32> = Vec::new();
    
        // Iterate over the captures of the regex in this line
        for cap_result in re.captures_iter(line) {
            // If the capture is successful, get the captures, otherwise print an error and continue to the next capture
            let cap = match cap_result {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to process line: {}", e);
                    continue;
                }
            };
    
            // Convert the captured group to a number
            let val = match cap.get(1).map(|m| m.as_str()) {
                Some("one") => 1,
                Some("two") => 2,
                Some("three") => 3,
                Some("four") => 4,
                Some("five") => 5,
                Some("six") => 6,
                Some("seven") => 7,
                Some("eight") => 8,
                Some("nine") => 9,
                Some(digit) => digit.parse::<u32>().unwrap(),
                None => continue,
            };
    
            // Add the number to the list of numbers for this line
            line_numbers.push(val);
        }
    
        // Add the list of numbers for this line to the list of all numbers
        numbers.push(line_numbers);
    }

    println!("Numbers:");
    for (i, numbers) in numbers.iter().enumerate() {
        println!("--------");
        println!("Line {}: ", i+1);
        println!("--------");
        println!("Original: {}", file_lines[i]);
        println!("Parsed: {:?}", numbers);

        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        let concatted = format!("{}{}", first, last).parse::<u32>().unwrap();

        println!("First: {}, Last: {}, Concatted: {}", first, last, concatted);
        println!("\n");

        total += concatted;
    }

    println!("Total: {}", total);
}