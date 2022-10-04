pub fn palindrome(input: &str) -> bool {
    let mut iter = input.chars();
    let input_len = input.len();

    for _ in 0..input_len {
        let front_char = iter.next();
        let back_char = iter.next_back();

        if front_char.is_some() && back_char.is_some() {
            if front_char != back_char {
                return false;
            }
        } else {
            // reached the pivot char and all others were equal:            
            return true;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aba_is_palindrome() {
        let input = "aba";
        let result = palindrome(&input);
        assert_eq!(result, true);
    }

    #[test]
    fn aba_space_before_not_palindrome() {
        let input = " aba";
        let result = palindrome(&input);
        assert_eq!(result, false);
    }

    #[test]
    fn aba_space_after_not_palindrome() {
        let input = "aba ";
        let result = palindrome(&input);
        assert_eq!(result, false);
    }

    #[test]
    fn greetings_not_palindrome() {
        let input = "greetings";
        let result = palindrome(&input);
        assert_eq!(result, false);
    }

    #[test]
    fn numeric_is_palindrome() {
        let input = "1000000001";
        let result = palindrome(&input);
        assert_eq!(result, true);
    }

    #[test]
    fn fish_different_casing_not_palindrome() {
        let input = "Fish hsif";
        let result = palindrome(&input);
        assert_eq!(result, false);
    }

    #[test]
    fn pennep_is_palindrome() {
        let input = "pennep";
        let result = palindrome(&input);
        assert_eq!(result, true);
    }
}
