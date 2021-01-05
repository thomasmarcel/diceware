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
}