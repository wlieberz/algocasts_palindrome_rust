# About

This repo contains my solution for a palindrome programming exercise.

There are many variations of this type of exercise, with varying levels of
complexity.

This particular variation comes from the Udemy course [The Coding Inteview Bootcamp: Algorithms + Data Structures](https://www.udemy.com/course/coding-interview-bootcamp-algorithms-and-data-structure/).

The companion repo for the course can be found here:
[StephenGrider/AlgoCasts](https://github.com/StephenGrider/AlgoCasts)

This solution is in Rust. ❤️ 🦀

# Problem Description

>--- Directions:
>
>Given a string, return true if the string is a palindrome
>or false if it is not.  Palindromes are strings that
>form the same word if it is reversed. 
>
> *Do* include spaces and punctuation in determining if the string is a palindrome.
>
>--- Examples:
>
>   palindrome("abba") === true
>
>   palindrome("abcdefg") === false

# Running Tests

Prerequisite: you should have the [Rust toolchain installed](https://www.rust-lang.org/tools/install).

In order to run the **unit-tests**:

```sh
cargo test
```
**Expected output:**
```sh
running 7 tests
test tests::aba_space_after_not_palindrome ... ok
test tests::aba_is_palindrome ... ok
test tests::aba_space_before_not_palindrome ... ok
test tests::fish_different_casing_not_palindrome ... ok
test tests::greetings_not_palindrome ... ok
test tests::numeric_is_palindrome ... ok
test tests::pennep_is_palindrome ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests palindrome

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```