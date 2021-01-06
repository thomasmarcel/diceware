use rand::Rng;

use crate::utils::read_lines;

#[derive(Debug)]
struct WordDict {
	dices: String,
	word: String,
}

pub fn process() -> () {
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
	for _i in 0..20 {
		println!("{:?}", roll_dice(6));
	}
}

fn roll_dice(times: u8) -> String {
	let mut s = String::new();
	for _time in 0..times {
		let number: u32 = rand::thread_rng().gen_range(1..7);
		s += &number.to_string();
	}
	s
}