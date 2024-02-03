use wordle_solver::{algorithms::Naive, Wordle};

const GAMES: &str = include_str!("../data/answers.txt");

fn main() {
    let wordle = Wordle::new();
    for answer in GAMES.lines() {
        let guesser = Naive::new();
        wordle.play(answer, guesser);
    }
}
