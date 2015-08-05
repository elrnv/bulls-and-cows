/**
 * bulls-and-cows Rust number guessing game
 * https://github.com/elrnv/bulls-and-cows
 *
 * Copyright 2015 Egor Larionov
 * 
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * 
 *     http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate rand;
extern crate num;

use std::io;
use rand::Rng;
use num::pow;

fn gen_unique_rand_digits(num: usize) -> Vec<u8> {
    assert!(num < 10);

    let mut all_digits = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut rng = rand::thread_rng(); // get local random number generator

    for i in 0..num {
        let r = rng.gen_range(i,10);
        if r != i { all_digits.swap(r,i); }
    }

    // return the generated digits
    all_digits[0..num].to_vec()
}

fn num_to_digits(num: u16, size: usize) -> Vec<u8> {
    let mut guess = num;
    let mut digits = vec![0u8; size];
    for i in (0..size).rev() {
        let denom = pow(10,i);
        digits[3-i] = (guess/denom) as u8;
        guess %= denom;
    }
    digits
}

// count digits in that are in the correct position
fn count_bulls(guess: &[u8], target: &[u8]) -> usize {
    target.iter().zip(guess.iter()).filter(|&(a,b)| a == b).count()
}

// count correct digits that in any position
fn count_correct(guess: &[u8], target: &[u8]) -> usize {
    let mut count = 0;
    for g in guess { for t in target { count += (g == t) as usize; } }
    count
}

// count correct digits that are in incorrect position
fn count_cows(guess: &[u8], target: &[u8]) -> usize {
    count_correct(guess,target) - count_bulls(guess,target)
}

// return true if digits in guess are unique
fn is_unique_digits(guess: &[u8]) -> bool {
    for i in 0..guess.len()-1 {
        for j in i+1..guess.len() {
            if guess[i] == guess[j] { return false; }
        }
    }
    true
}

fn main() {
    let num_digits = 4usize;

    println!("Welcome to Bulls and Cows!");
    println!("The goal of the game is to guess a number with {} unique digits correctly in the\n\
              least number of steps. Valid digits are from 0 to 9 inclusive. Bulls are digits\n\
              you guess correctly, and Cows are digits that are correct but in wrong location.", num_digits);

    let target = gen_unique_rand_digits(num_digits);

    println!("Enter your guesses below as a contiguous 4 digit number with non-repeating digits.");

    let mut moves = 1;
    'm: loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read from standard input.");

        let four_digit_msg = || { println!("Please enter a {} digit number!", num_digits); };
        let unique_digit_msg = || { println!("Please enter {} unique digits!", num_digits); };

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { four_digit_msg(); continue; }, // catch error later
        };

        let digits = num_to_digits(guess, num_digits);
        for d in digits.iter() {
            if *d > 9u8 { four_digit_msg(); continue 'm; }
        }
        if !is_unique_digits(&digits) { unique_digit_msg(); continue 'm; }

        let bulls = count_bulls(&digits, &target);
        let cows  = count_cows(&digits, &target);

        if bulls == num_digits  {
            println!("Congratulations! You won in {} move{}!", moves, if moves > 1 { 's' } else { ' ' });
            break;
        } else {
            print!("      "); // space for clarity
            for d in digits.iter() { print!("{} ", d) };
            println!("has {}b {}c", bulls, cows);
            moves += 1;
        }
    }
}
