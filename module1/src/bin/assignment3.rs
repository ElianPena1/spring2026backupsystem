fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Secret number
    let mut secret: i32 = 42;

    // track guesses
    let mut guess: i32 = 0;
    let mut guess_count: i32 = 0;

    loop {
        guess_count = guess_count + 1;

        //  guesses 
        if guess_count == 1 {
            guess = 10;
        } else if guess_count == 2 {
            guess = 50;
        } else if guess_count == 3 {
            guess = 40;
        } else {
            guess = 42;
        }

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess #{guess_count}: {guess} is correct!");
            break;
        } else if result == 1 {
            println!("Guess #{guess_count}: {guess} is too high.");
        } else {
            println!("Guess #{guess_count}: {guess} is too low.");
        }
    }

    println!("It took {guess_count} guesses.");
}
