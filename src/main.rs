use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use termion::color;

enum Functionality {
    Assistant,
    Player,
}

fn main() {
    println!(
        "\n\nHello, world! Welcome to our {}Wordle Solver{}, make yourself at home.\n",
        color::Fg(color::LightMagenta),
        color::Fg(color::Reset)
    );
    let mut five_letter_dict = create_dict();
    let funct = get_start_input();

    let mut user_input;
    for i in 1..6 {
        match funct {
            Functionality::Assistant => print_list(&five_letter_dict),
            Functionality::Player => choose_word(&five_letter_dict),
        };
        user_input = get_input(i);
        five_letter_dict = sort_dict(user_input.chars().collect(), five_letter_dict.clone());
        user_input.clear();
    }
}

fn get_start_input() -> Functionality {
    let functionality: Functionality;
    let mut user_input = String::new();
    loop {
        println!("Type {}player{} if you want the program to choose the word for you (We won't judge you).\nType {}assistant{} if you just want a list of suggestions.\n", color::Fg(color::Yellow), color::Fg(color::Reset),  color::Fg(color::Yellow), color::Fg(color::Reset));
        io::stdin().read_line(&mut user_input).unwrap();
        if user_input == "player\n" {
            functionality = Functionality::Player;
            break;
        } else if user_input == "assistant\n" {
            functionality = Functionality::Assistant;
            break;
        }
        println!(
            "{}Please, try again...{}",
            color::Fg(color::Red),
            color::Fg(color::Reset)
        );
        user_input.clear();
    }
    return functionality;
}

fn get_input(i: usize) -> String {
    let mut user_input = String::new();
    loop {
        println!("\n{}Attempt No {i}{}\n  Type your guess in the following format -> {}their00120{}\n  The 5 letters will be your guess\n  The 5 numbers will correspond to each one of the letters and represent the output of the game.\n0 means that the letter in not in the word.\n1 means that the letter is in the word but in a wrong spot.\n2 means that the letter is in the word and in the right place.\n\nType {}done{} if you guessed correctly. Type {}exit{} to quit the program.\n", color::Fg(color::LightYellow), color::Fg(color::Reset), color::Fg(color::Yellow), color::Fg(color::Reset), color::Fg(color::Yellow), color::Fg(color::Reset),color::Fg(color::Yellow), color::Fg(color::Reset));
        io::stdin().read_line(&mut user_input).unwrap();
        if user_input == "done\n" {
            println!(
                "{}Congratulations!\n{}",
                color::Fg(color::LightYellow),
                color::Fg(color::Reset)
            );
            exit(0);
        } else if user_input == "exit\n" {
            println!(
                "{}Bye!{}",
                color::Fg(color::LightYellow),
                color::Fg(color::Reset)
            );
            exit(0);
        } else {
            if correct_input(&user_input) {
                break;
            }
        }
        println!(
            "{}Your input was not in the specified format.{}",
            color::Fg(color::Red),
            color::Fg(color::Reset)
        );
        user_input.clear();
    }
    return user_input;
}

fn correct_input(user_input: &str) -> bool {
    user_input.len() == 11
        && user_input[0..5].chars().all(char::is_alphabetic)
        && user_input[5..10].chars().all(num_is_correct)
        && user_input.chars().collect::<Vec<char>>()[10] == '\n'
}

fn num_is_correct(c: char) -> bool {
    c == '0' || c == '1' || c == '2'
}

fn print_list(list: &Vec<String>) {
    let path = Path::new("guesses/guesses.txt");
    let mut file = File::create(&path).unwrap();

    for word in list {
        file.write_all(word.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }

    let step = list.len() / 10;
    if list.len() > 10 {
        println!("\nThere are {}{}{} valid words.\n\nThese are {}10{} random words, you can access the whole list in {}guesses/guesses.txt{}:", color::Fg(color::LightCyan),list.len(), color::Fg(color::Reset), color::Fg(color::LightCyan), color::Fg(color::Reset), color::Fg(color::Yellow), color::Fg(color::Reset));
        for i in 0..10 {
            println!("{}", list[i * step]);
        }
    } else {
        println!(
            "\nThere are {}{}{} words left:",
            color::Fg(color::LightCyan),
            list.len(),
            color::Fg(color::Reset)
        );
        for item in list {
            println!("{}", item);
        }
    }
}

fn sort_dict(user_input: Vec<char>, mut words: Vec<String>) -> Vec<String> {
    let mut green_dict: HashMap<usize, char> = HashMap::new();
    let mut yellow_dict: HashMap<usize, char> = HashMap::new();
    let mut grey_dict: HashMap<usize, char> = HashMap::new();
    for i in 0..5 {
        if user_input[i + 5] == '2' {
            green_dict.insert(i, user_input[i]);
        } else if user_input[i + 5] == '1' {
            yellow_dict.insert(i, user_input[i]);
        } else {
            grey_dict.insert(i, user_input[i]);
        }
    }
    for (index, letter) in &green_dict {
        words = words
            .into_iter()
            .filter(|s| {
                let temp = s.chars().position(|c| c == *letter);
                if let Some(i) = temp {
                    return i == *index;
                } else {
                    return false;
                }
            })
            .collect();
    }
    for (index, letter) in &yellow_dict {
        let mut indexes = vec![0, 1, 2, 3, 4];
        indexes.retain(|f| f != index);
        for (green_index, green_letter) in &green_dict {
            if letter == green_letter {
                indexes.retain(|f| f != green_index);
            }
        }
        words = words
            .into_iter()
            .filter(|s| {
                for i in &indexes {
                    if (*s).chars().collect::<Vec<char>>()[*i] == *letter {
                        return true;
                    }
                }
                return false;
            })
            .collect();
    }
    for (_index, letter) in &grey_dict {
        let amount = green_dict.values().filter(|v| *v == letter).count()
            + yellow_dict.values().filter(|v| *v == letter).count();
        words = words
            .into_iter()
            .filter(|s| (*s).chars().filter(|c| c == letter).count() == amount)
            .collect();
    }
    return words;
}

fn choose_word(list: &Vec<String>) {
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut letter_frequency: HashMap<char, usize> = HashMap::new();

    for letter in alphabet.chars() {
        let value = list.iter().filter(|w| (*w).contains(letter)).count();
        letter_frequency.insert(letter, value);
    }

    let mut best_word: (String, usize) = (String::new(), 0);
    for w in list.iter() {
        let mut letters = w.chars().collect::<Vec<char>>();
        letters.sort();
        letters.dedup();

        let mut rating: usize = 0;
        for char in &letters {
            rating += letter_frequency[char];
        }
        if rating > best_word.1 {
            best_word.0 = w.to_string();
            best_word.1 = rating;
        }
    }
    println!(
        "\nOur council of Elders have determined that you must type {}{}{} in Wordle",
        color::Fg(color::Magenta),
        best_word.0,
        color::Fg(color::Reset)
    );
}

fn create_dict() -> Vec<String> {
    let dict: Vec<String> = read_to_string("assets/wordle_dict.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| s.len() == 5)
        .collect();
    return dict;
}
