mod guess_number;

fn main() {
    println!("Number guessing game (reversed)");

    println!(
        "Input a secret number between 1 and {}:",
        guess_number::MAX_NUMBER
    );

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let secret_number: u32 = match input.trim().parse() {
        Ok(num) if num <= guess_number::MAX_NUMBER && num > 0 => num,
        Ok(_) => {
            println!(
                "Please enter a number between 1 and {}",
                guess_number::MAX_NUMBER
            );
            return;
        }
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("Attempting to guess the number via binary search...");

    let attempts = guess_number::guess_number(secret_number, guess_number::MAX_NUMBER / 2, 1);
    println!("The secret number was {secret_number}.");
    println!("It took {attempts} tries to guess the number.");
}
