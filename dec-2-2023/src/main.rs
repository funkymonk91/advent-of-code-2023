use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn default() -> Self {
        Self {
            red: 12,
            green: 13,
            blue: 14,
        }
    }

    fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
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

    fn minimum_cubes(&self) -> Cubes {
        let mut min_cubes = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };

        for pull in &self.pulls {
            for cube in pull {
                if min_cubes.red < cube.red {
                    min_cubes.red = cube.red;
                }
                if min_cubes.green < cube.green {
                    min_cubes.green = cube.green;
                }
                if min_cubes.blue < cube.blue {
                    min_cubes.blue = cube.blue;
                }
            }     
        }

        return min_cubes
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
    let mut sum_of_ids: u32 = 0; 
    let mut sum_min_power: u32 = 0;

    for line in file_lines {
        let game = Game::from_line(line);
        if game.possible() {
            // println!("Game: {} possible", game.id);
            sum_of_ids += game.id;
        }

        let min_cubes = game.minimum_cubes();
        println!("Game: {} minimum cubes: {:?}", game.id, min_cubes);
        sum_min_power += min_cubes.power();

        games.push(game);
    }

    println!("Sum of possible ids: {}", sum_of_ids);
    println!("Sum of minimum power: {}", sum_min_power);
}