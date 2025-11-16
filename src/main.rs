use clap::Parser;

// Run specific day's AoC solution.
#[derive(Parser)]
struct Args {
    // Day to solve 
    day: usize, 

    // Path to input file to override the provided puzzle input
    #[arg(short, long)]
    input_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input_file: String = args.input_file.unwrap_or_else(|| {
        format!("inputs/day{}.txt", args.day)
    });

    match args.day {
        1 => { day01::solve(&input_file) },
        2 => { day02::solve(&input_file) },
        3 => { day03::solve(&input_file) },
        4 => { day04::solve(&input_file) },
        _ => { println!("Haven't solved day {} yet!", args.day) },
    }
}


mod day01;
mod day02;
mod day03;
mod day04;
