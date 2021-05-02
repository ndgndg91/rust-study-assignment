extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::collections::LinkedList;
use std::iter::FromIterator;
use std::borrow::Borrow;

fn main() {
    // guess_game();
    // sum_1_to_100();
    // reverse_input();
    twenty_numbers();
}

#[allow(dead_code)]
fn twenty_numbers() {
    loop {
        let inputs = get_inputs();

        if inputs.trim().len() > 16 {
            println!("input length exceed!");
            continue;
        }

        let inputs: u64 = match inputs.trim().parse() {
            Ok(number) => number,
            Err(message) => {
                println!("{}", message);
                continue;
            }
        };

        let inputs_string = inputs.to_string();

        let mut buf: LinkedList<char> = LinkedList::new();
        let reverse_inputs: String = inputs_string.chars().rev().collect();
        let chunk_string: Vec<&[u8]> = reverse_inputs.as_bytes().chunks(3).collect();
        for ch in chunk_string {
            for c in ch {
                let c = char::from(*c);
                buf.push_front(c);
            }
            buf.push_front(',');
        }

        let mut char_vec: Vec<char> = vec![];
        for i in 0..buf.len() {
            if i == 0 {
                let first = buf.pop_front().unwrap();
                if first == ',' {
                    continue;
                }
                let x = first;
                char_vec.push(x);
            }
            let x = buf.pop_front().unwrap();
            char_vec.push(x);
        }

        let formatted_numbers = String::from_iter(char_vec);
        println!("{}", formatted_numbers);
        let zero_count = 20 - formatted_numbers.len();
        let result = "0".repeat(zero_count) + formatted_numbers.borrow();
        println!("{}", result);

        break;
    }
}

fn get_inputs() -> String {
    println!("Please input something");

    let mut inputs = String::new();

    io::stdin()
        .read_line(&mut inputs)
        .expect("Failed to read line");

    return inputs;
}

#[allow(dead_code)]
fn reverse_input(){
    let inputs = get_inputs();

    reverse(&inputs);
    reverse_second(&inputs);
}

fn reverse(inputs: &str) {
    let inputs = String::from(inputs.trim());
    let input_length = inputs.len();
    let mut reverse_inputs: Vec<u8> = vec![];
    for i in (0..input_length).rev() {
        match inputs.as_bytes().get(i) {
            Some(c) => {
                reverse_inputs.push(*c);
            }
            None => println!("none")
        }
    }

    let result = String::from_utf8(reverse_inputs).expect("can not convert");
    println!("{}", result);
}

fn reverse_second(inputs: &str) {
    let result: String = inputs.chars().rev().collect();
    println!("{}", result);
}

#[allow(dead_code)]
fn sum_1_to_100(){
    let mut sum = 0;
    for num in 1..100 {
        sum += num;
    }

    println!("sum of 1 to 100 : {}", sum);
}

#[allow(dead_code)]
fn guess_game(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess number.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falied to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(message) => {
                println!("{}", message);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number is : {}", secret_number);
                break;
            }
        }
    }
}