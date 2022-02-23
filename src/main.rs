use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
	let five_letter_dict = create_dict();
	println!("{}", five_letter_dict.len());
	for i in 0..50 {
		println!("{}", five_letter_dict[i * 500]);
	}
}

fn create_dict() -> Vec<String> {
	let dict: Vec<String> = read_to_string("assets/en_dict.txt").unwrap()
	.split("\n")
	.map(|s| s.to_string())
	.filter(|s| s.len() == 5).collect();
	return dict;
}
