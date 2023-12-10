use cli_table::{format::Justify, print_stdout, Table, WithTitle};

#[derive(Table)]
struct Puzzle {
    #[table(title = "Date")]
    date: &'static str,
    #[table(title = "First", justify = "Justify::Center")]
    first: &'static str,
    #[table(title = "Second" , justify = "Justify::Center")]
    second: &'static str,
}

fn main() {
    let data = vec![
        Puzzle {
            date: "December 1 2023",
            first: "✅",
            second: "✅",
        },
        Puzzle {
            date: "December 2 2023",
            first: "✅",
            second: "✅",
        },
        Puzzle {
            date: "December 3 2023",
            first: "✅",
            second: "✅",
        },
        Puzzle {
            date: "December 4 2023",
            first: "✅",
            second: "",
        },
        Puzzle {
            date: "December 5 2023",
            first: "",
            second: "",
        },
        Puzzle {
            date: "December 6 2023",
            first: "",
            second: "",
        },
        Puzzle {
            date: "December 7 2023",
            first: "",
            second: "",
        },
        Puzzle {
            date: "December 8 2023",
            first: "",
            second: "",
        },
        Puzzle {
            date: "December 9 2023",
            first: "",
            second: "",
        },
    ];

    print_stdout(data.with_title());
    
    
}
