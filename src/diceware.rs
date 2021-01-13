use rand::Rng;

use crate::utils::read_lines;

#[derive(Debug)]
struct WordDict {
	dices: String,
	word: String,
}

pub fn process() -> () {
	let word_num = 2;

	let special_chars = vec![
		"#",
		"-",
		"@",
		"=",
		"+", 
		"%",
		"*",
		"?",
		":",
		".",
		"!",
		"&",
		"$"
	];

	let mut selected_special_chars: Vec<String> = Vec::new();
	for _n in 0..(word_num - 1) {
		let i = rand::thread_rng().gen_range(0..special_chars.len());
		selected_special_chars.push(String::from(special_chars[i]));
	}

	let mut rand_nums: Vec<u32> = Vec::new();
	for _n in 0..(word_num - 1) {
		let rand_num: u32 = rand::thread_rng().gen_range(0..9);
		rand_nums.push(rand_num);
	}

	let mut dict: Vec<WordDict> = Vec::new();
	if let Ok(lines) = read_lines("diceware.wordlist.asc") {
		for line in lines {
			if let Ok(row) = line {
				let row_list: Vec<&str> = row.split('\t').collect();
				if row_list.len() == 2 {
					let row_dict = WordDict {
						dices: String::from(row_list[0]),
						word: String::from(row_list[1])
					};
					dict.push(row_dict);
				}
			}
		}
	}
	println!("{:?}", dict);
	let mut word_dices: Vec<String> = Vec::new();
	for _i in 0..word_num {
		word_dices.push(roll_dice(5));
	}

	let mut words_found = 0;
	let mut index = 0;
	let mut words: Vec<String> = Vec::new();
	while words_found < word_dices.len() && index < dict.len() {
		let cur_word = &dict[index];
		for dices in &word_dices {
			if cur_word.dices == *dices {
				words.push(String::from(&cur_word.word));
				words_found += 1;
			}
		}
		index += 1;
	}

	println!("{:?}", &word_dices);
	println!("{:?}", &selected_special_chars);
	println!("{:?}", &rand_nums);
	println!("{:?}", &words);
}

fn roll_dice(times: u8) -> String {
	let mut s = String::new();
	for _time in 0..times {
		let number: u32 = rand::thread_rng().gen_range(1..7);
		s += &number.to_string();
	}
	s
}