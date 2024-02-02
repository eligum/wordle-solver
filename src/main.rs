use wordle_solver::algorithms::Naive;

const GAMES: &str = include_str!("../data/answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = Naive::new();
        wordle_solver::play(answer, guesser);
    }
}
