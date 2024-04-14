use rand::Rng;

pub struct NumberGuessingGaming {
    correct_answer: i64,
    attempts: i64,

    guess_range_start: i64,
    guess_range_end: i64,
}


impl NumberGuessingGaming {
    pub fn new() -> NumberGuessingGaming {
        NumberGuessingGaming {
            correct_answer: 0,
            attempts: 0,

            guess_range_start: 0,
            guess_range_end: 9
        }   
    }

    pub fn prompt_guess(&self) {
        let range_start = self.guess_range_start;
        let range_end = self.guess_range_end;

        println!("The number is between {range_start} and {range_end}");
    }

    pub fn new_number(&mut self) {
        self.correct_answer = rand::thread_rng().gen_range(self.guess_range_start..self.guess_range_end);
        self.attempts = 0;
    }


    pub fn guess_number(&mut self, guess: i64) -> bool {
        if guess == self.correct_answer {
            let attempts = self.attempts;

            println!("You guessed correctly! It was {guess}!\nIt took you {attempts} attempt(s) to guess!");

            self.new_number();

            return true
        } else {
            self.attempts += 1;
        }

        return false
    }
}

