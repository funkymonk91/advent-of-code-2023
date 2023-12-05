use std::fs;
use regex::Regex;

#[derive(Debug)]
struct PartNumber {
    value: i32,
    line: usize,
    index: usize,
    length: usize,
    range: std::ops::Range<usize>,
}

#[derive(Debug)]
struct SymbolPosition {
    value: String,
    line: usize,
    index: usize,
}

fn main() {
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    let file_lines: Vec<_> = contents.lines().collect();

    let mut total = 0;
    
    let re_number = Regex::new(r"\d+").unwrap();
    let re_symbol = Regex::new(r"[/@$=&#-+%]").unwrap();

    let mut numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<SymbolPosition> = Vec::new();

    for (i, line) in file_lines.iter().enumerate() {
        for cap in re_number.captures_iter(line) {
            let number = cap[0].parse::<i32>().unwrap();
            let index = cap.get(0).unwrap().start();
            let length = number.to_string().len();

            let start = if index > 0 { index - 1 } else { index };
            let end = index + number.to_string().len() + 1;

            let part_number = PartNumber {
                value: number,
                line: i,
                index: index,
                length: length,
                range: start..end,
            };
            numbers.push(part_number);
        }

        for cap in re_symbol.captures_iter(line) {
            let symbol_position = SymbolPosition {
                value: cap[0].to_string(),
                line: i,
                index: cap.get(0).unwrap().start(),
            };
            symbols.push(symbol_position);
        }
    }

    for symbol in symbols {
        // only get numbers that are either on the line before, the same line, or the line after the symbol
        let potential_numbers: Vec<&PartNumber> = numbers.iter().filter(|&x| x.line == symbol.line || x.line == symbol.line - 1 || x.line == symbol.line + 1).collect();

        println!("SYMBOL: {:?}", symbol);
        // check to see if number is adjacent to symbol
        for number in &potential_numbers {
            let mut adjacent = false;

            // LINE BEFORE
            if symbol.line > 0 && symbol.line - 1 == number.line {
                if number.range.contains(&symbol.index) {
                    println!("LINE BEFORE: Number: {:?}", number);
                    adjacent = true;
                }
            }
            // SAME LINE
            if number.line == symbol.line {
                if number.range.contains(&symbol.index) {
                    println!("SAME LINE: Number: {:?}", number);
                    adjacent = true;
                }
            }
            // LINE AFTER
            if symbol.line < file_lines.len() - 1 && symbol.line + 1 == number.line {
                if number.range.contains(&symbol.index) {
                    println!("LINE AFTER: Number: {:?}", number);
                    adjacent = true;
                }
            }

            if adjacent {
                total += number.value;
            }
        }
        
        println!("---");
    }

    println!("Total: {}", total);
}
