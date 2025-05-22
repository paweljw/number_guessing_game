use std::cmp::Ordering;

/// The maximum number that can be guessed.
pub const MAX_NUMBER: u32 = 100;

/// Attempts to guess the number via binary search.
///
/// # Arguments
///
/// * `secret_number` - The number to guess.
/// * `guess` - The current guess.
/// * `attempt` - The current attempt.
///
/// # Returns
/// The number of attempts it took to guess the number.
///
/// # Examples
/// ```
/// let attempts = guess_number(10, 5, 1);
/// assert_eq!(attempts, 2);
/// ```
pub fn guess_number(secret_number: u32, guess: u32, attempt: u32) -> u32 {
    let step = if MAX_NUMBER / (2_u32.pow(attempt + 1)) > 0 {
        MAX_NUMBER / (2_u32.pow(attempt + 1)) + 1
    } else {
        1
    };

    println!(
        "Guess: {guess}, Half of guess: {step}, Attempt: {attempt}, Secret number: {secret_number}"
    );

    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            return guess_number(secret_number, guess + step, attempt + 1);
        }
        Ordering::Greater => {
            println!("Too big!");
            return guess_number(secret_number, guess - step, attempt + 1);
        }
        Ordering::Equal => {
            return attempt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_number_lower_bound() {
        // Secret number is the lowest possible
        let attempts = guess_number(1, MAX_NUMBER / 2, 1);
        assert!(attempts > 0);
    }

    #[test]
    fn test_guess_number_upper_bound() {
        // Secret number is the highest possible
        let attempts = guess_number(MAX_NUMBER, MAX_NUMBER / 2, 1);
        assert!(attempts > 0);
    }

    #[test]
    fn test_guess_number_middle() {
        // Secret number is in the middle
        let attempts = guess_number(MAX_NUMBER / 2, MAX_NUMBER / 2, 1);
        assert_eq!(attempts, 1);
    }

    #[test]
    fn test_guess_number_just_above_initial() {
        // Secret number is just above the initial guess
        let attempts = guess_number((MAX_NUMBER / 2) + 1, MAX_NUMBER / 2, 1);
        assert!(attempts > 1);
    }

    #[test]
    fn test_guess_number_just_below_initial() {
        // Secret number is just below the initial guess
        let attempts = guess_number((MAX_NUMBER / 2) - 1, MAX_NUMBER / 2, 1);
        assert!(attempts > 1);
    }

    #[test]
    fn test_guess_number_attempts_reasonable() {
        // For binary search, attempts should not exceed log2(MAX_NUMBER) + 1
        let max_attempts = ((MAX_NUMBER as f64).log2().ceil() as u32) + 1;
        for secret in [
            1,
            MAX_NUMBER,
            MAX_NUMBER / 2,
            (MAX_NUMBER / 2) + 1,
            (MAX_NUMBER / 2) - 1,
        ] {
            let attempts = guess_number(secret, MAX_NUMBER / 2, 1);
            assert!(
                attempts <= max_attempts,
                "Too many attempts for secret {}: {}",
                secret,
                attempts
            );
        }
    }
}
