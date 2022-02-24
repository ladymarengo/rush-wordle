use std::fs::read_to_string;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::process::exit;
// use termion::{color, style};

fn main() {
    println!("Hello, world! Welcome to our Worldle assistant, make yourself at home.\n");
	let mut five_letter_dict = create_dict();
	println!("We have {} words in our dictionary, you can check them all in assets/en_dict.txt!", five_letter_dict.len());

	let mut user_input;
	for i in 1..6 {
		// println!("\nPlease write your {i} try here in format like 'their00120' where 0 means not present, 1 is present but not in place and 2 is in the right place.");
		user_input = get_input(i);
		// println!("You printed {}", user_input);
		five_letter_dict = sort_dict(user_input.chars().collect(), five_letter_dict.clone());
		println!("\nThere are {} words left.\n\nThese are 10 random words, you can access whole list in 'guesses/guesses.txt':", five_letter_dict.len());
		print_list(&five_letter_dict);
		// for i in 0..10 {
		// 	println!("{}", five_letter_dict[i]);
		// }
		user_input.clear();
	}
}

fn get_input(i: usize) -> String {
	let mut user_input = String::new();
	loop {
		println!("\nAttempt No {i}\n  Type your guess in the following format -> 'their00120'\n  The 5 letters will be your guess\n  The 5 numbers will correspond to each one of the letters and represent the output of the game.\n0 means that the letter in not in the word.\n1 means that the letter is in the word but in a wrong spot.\n2 means that the letter is in the word and in the right place.\n\nType 'done' if you guessed correctly. Type 'exit' to quit the program.");
		io::stdin().read_line(& mut user_input).unwrap();
		if user_input == "done\n" {
			println!("Congratulations!");
			exit(0);
		}
		else if user_input == "exit\n" {
			println!("Bye!");
			exit(0);
		}
		else {
			if correct_input(&user_input) {
				break;
			}
		}
		println!("Your input was not in correct format.");
		user_input.clear();
	}
	return user_input;
}

fn correct_input(user_input: &str) -> bool {
	user_input.len() == 11 && user_input[0..5].chars().all(char::is_alphabetic) && user_input[5..10].chars().all(num_is_correct) && user_input.chars().collect::<Vec<char>>()[10] == '\n'
}

fn num_is_correct(c: char) -> bool {
	c == '0' || c == '1' || c == '2'
}

fn print_list(list: &Vec<String>) {
	let path = Path::new("guesses/guesses.txt");
	// let display = path.display();

	let mut file = File::create(&path).unwrap();

	for word in list {
		file.write_all(word.as_bytes()).unwrap();
		file.write_all("\n".as_bytes()).unwrap();
	}

	let step = list.len() / 10;
	for i in 0..10 {
		println!("{}", list[i * step]);
	}
}

fn sort_dict(user_input: Vec<char>, mut words: Vec<String>) -> Vec<String> {
	let mut input_dict: HashMap<char, (char, usize)> = HashMap::new();
	for i in 0..5 {
		input_dict.insert(user_input[i], (user_input[i + 5], i));
	}
	println!("{:?}", input_dict);
	for (letter, (state, index)) in &input_dict {
		if *state == '0' {
			words = words.into_iter().filter(|s| !(*s).contains(*letter)).collect();
		}
		else if *state == '2' {
			words = words.into_iter().filter(|s| {
				let temp = s.chars().position(|c| c == *letter);
				if let Some(i) = temp {
					return i == *index;
				}
				else {
					return false;
				}
			}).collect();
		}
		else if *state == '1' {
			// words = words.into_iter().filter(|s| (*s).contains(*letter)).collect();
			words = words.into_iter().filter(|s| {
				let temp = s.chars().position(|c| c == *letter);
				if let Some(i) = temp {
					return i != *index;
				}
				else {
					return false;
				}
			}).collect();
		}
	}
	return words;
}

 fn create_dict() -> Vec<String> {
 	let dict: Vec<String> = read_to_string("assets/en_dict.txt").unwrap()
 	.split("\n")
 	.map(|s| s.to_string())
 	.filter(|s| s.len() == 5).collect();
 	return dict;
 }

//fn create_dict() -> Vec<String> {
//	let dict: Vec<String> = read_to_string("../assets/frequency.txt").unwrap()
//	.split("\n")
//	.map(|s| s.to_string())
//	.filter(|s| {
//		let n: Vec<String> = s.split("\t").map(|s| s.to_string()).collect();
//		return n[0].len() == 5}).collect();
//	return dict;
//}


//"THEIR" is the most common word with all different most common letters in english
//So it is the perfect start without data

//An easy program can do:
//Ask the player to enter "their" as first word
//Ask to write 0 in place of letters not present
//Write 1 in place of letters present in wrong place
//Write 2 in place of correct letters in right place
// Ex:  THEIR -> 00100
//      WOMEN -> 01010
//      ABOVE -> 02202


//Ask what letters are in the word in undeterminated place
//Ask what letters are right in place
//Ask what letters are not present (after the first round only)
