use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Default for Cubes {
    fn default() -> Self {
        Self {
            red: 12,
            green: 13,
            blue: 14,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Vec<Cubes>>,
}

impl Game {
    fn possible(&self) -> bool {
        let max_cubes = Cubes::default();

        for pull in &self.pulls {
            for cube in pull {
                if cube.red > max_cubes.red 
                    || cube.green > max_cubes.green 
                    || cube.blue > max_cubes.blue {
                    return false;
                }
            }     
        }

        return true
    }

    fn from_line(line: &str) -> Game {
        // ex) Game 1: 4 red, 5 blue, 9 green; 7 green, 7 blue, 3 red; 16 red, 7 blue, 3 green; 11 green, 11 blue, 6 red; 12 red, 14 blue
        let re_game = Regex::new(r"[Ga][Aa][Mm][Ee] ?(\d+).?:").unwrap();
        let reRed = Regex::new(r"(\d+) ?([Rr][Ee][Dd]);?").unwrap();
        let reGreen = Regex::new(r"(\d+) ?([Gg][Rr][Ee][Ee][Nn]);?").unwrap();
        let reBlue = Regex::new(r"(\d+) ?([Bb][Ll][Uu][Ee]);?").unwrap();
    
        let mut game = Game {
            id: re_game.captures(line).unwrap()[1].parse().unwrap(),
            pulls: Vec::new(),
        };

        let mut line = re_game.replace(line, "");
        let pulls: Vec<_> = line.split(";").map(|i| i.trim()).collect();

        for pull in pulls {
            // println!("Pull: {}", pull);

            let cubes = Cubes {
                red: reRed.captures(pull).and_then(|cap| cap[1].parse().ok()).unwrap_or(0),
                green: reGreen.captures(pull).and_then(|cap| cap[1].parse().ok()).unwrap_or(0),
                blue: reBlue.captures(pull).and_then(|cap| cap[1].parse().ok()).unwrap_or(0),
            };
            
            game.pulls.push(vec![cubes]);
        }

        game
    }
}


fn main() {
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    let file_lines: Vec<_> = contents.lines().collect();

    let bag = Cubes::default();
    println!("Bag {:?}", bag);
    let mut games: Vec<Game> = Vec::new();
    let mut sumOfIds: u32 = 0;    

    for line in file_lines {
        let game = Game::from_line(line);
        if game.possible() {
            // sumOfIds += game.id;
            println!("Game: {} possible", game.id);
            sumOfIds += game.id;
        }
        games.push(game);
    }

    println!("Sum of possible ids: {}", sumOfIds);
}