use cli_table::{format::Justify, print_stdout, Table, WithTitle};

#[derive(Table)]
struct Puzzle {
    #[table(title = "Puzzle")]
    name: &'static str,
    #[table(title = "First", justify = "Justify::Center")]
    first: &'static str,
    #[table(title = "Second" , justify = "Justify::Center")]
    second: &'static str,
}

fn main() {
    let data = vec![
        Puzzle {
            name: "Day 1: Trebuchet?!",
            first: "✅",
            second: "✅",
        },
        Puzzle {
            name: "Day 2: Cube Conundrum",
            first: "✅",
            second: "✅",
        },
        Puzzle {
            name: "Day 3: Gear Ratios",
            first: "✅",
            second: "✅",
        },
        Puzzle {
            name: "Day 4: Scratchcards",
            first: "✅",
            second: "✅",
        },
        Puzzle {
            name: "Day 5: If You Give A Seed A Fertilizer",
            first: "✅",
            second: "✅",
        },
        Puzzle {
            name: "Day 6: Wait For It",
            first: "✅",
            second: "✅",
        },
        Puzzle {
            name: "Day 7: Camel Cards",
            first: "",
            second: "",
        },
        Puzzle {
            name: "Day 8: Haunted Wasteland",
            first: "",
            second: "",
        },
        Puzzle {
            name: "Day 9: Mirage Maintenance",
            first: "",
            second: "",
        },
        Puzzle {
            name: "Day 10: Pipe Maze",
            first: "",
            second: "",
        },
    ];

    print_stdout(data.with_title()).unwrap();
}
