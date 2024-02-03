use std::collections::HashSet;

pub mod algorithms;

const MAX_GUESSES: usize = 32;
const DICTIONARY: &str = include_str!("../data/dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines()),
        }
    }

    /// Play the game infinitely until the guesser guessess the correct word.
    pub fn play<G: Guesser>(&self, answer: &str, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();
        // Wordle only allows siz guesses.
        // We allow more to avoid choping off the score distribution for stats purposes.
        for i in 1..=MAX_GUESSES {
            let guess = guesser.guess(&history[..]);
            if guess == answer {
                return Some(i);
            }
            assert!(
                self.dictionary.contains(&*guess),
                "guess '{}' is not in the dictionary",
                guess
            );
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: guess,
                mask: correctness,
            });
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    /// Todo.
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        // Mark everything gray by default
        let mut cor = [Correctness::Wrong; 5];
        // Mark things green
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                cor[i] = Correctness::Correct;
            }
        }
        // Mark things yellow
        let mut used = [false; 5];
        for i in 0..cor.len() {
            if cor[i] == Correctness::Correct {
                used[i] = true;
            }
        }
        for (i, g) in guess.chars().enumerate() {
            if cor[i] == Correctness::Correct {
                continue;
            }
            if answer.chars().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                cor[i] = Correctness::Misplaced;
            }
        }
        cor
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

impl Guesser for fn(history: &[Guess]) -> String {
    fn guess(&mut self, history: &[Guess]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
mod tests {
    mod game {
        use crate::Wordle;

        #[test]
        fn play() {
            let wordle = Wordle::new();
        }
    }

    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C]);
        }

        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute("abcde", "bcdea"), mask![M M M M M]);
        }

        #[test]
        fn all_gray() {
            assert_eq!(Correctness::compute("abcde", "fghij"), mask![W W W W W]);
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask![C C W W W]);
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute("aabbb", "ccaac"), mask![W W M M W]);
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute("aabbb", "acaac"), mask![C W M W W]);
        }
    }
}
