use std::collections::HashSet;
use std::fs;

struct GameState {
	life_point: i8,
	guessed_characters: HashSet<char>,
}

fn input() -> Result<String, &'static str> {
	let mut buffer = String::new();
	match std::io::stdin().read_line(&mut buffer) {
		Ok(x) => Ok(buffer),
		Err(x) => Err("Error"),
	}
}

fn main() {
	let content = fs::read_to_string("words.txt")
        .expect("Cannot find the file in the current directory");
	let words = content.split("\n").map(|x| x.to_lowercase()).collect::<Vec<String>>();
	let random_number = (rand::random::<f32>() * words.len() as f32) as usize;
	let choosed_word = &words[random_number];

	let mut state = GameState {
		life_point: 5,
		guessed_characters: HashSet::new(),
	};

	loop {
		println!("You have {} chance left to guess", state.life_point);
		println!("Guessed but invalid characters: {}", state.guessed_characters
			.iter()
			.filter(|x| !choosed_word.contains(&x.to_string()))
			.map(|x| x.to_string())
			.collect::<Vec<String>>()
			.join(", "));
		println!("{}", choosed_word
			.chars()
			.map(|x| {
				if state.guessed_characters.contains(&x) {
					format!("{} ", x).to_owned()
				} else {
					"_ ".to_string()
				}
			})
			.collect::<String>());
		let user_input = input();
		match user_input {
			Ok(x) => {
				match x.to_lowercase().chars().nth(0) {
					Some(letter) => {
						state.guessed_characters.insert(letter);
						if !choosed_word.contains(&letter.to_string()) {
							if state.life_point <= 1 {
								println!("You failed, try harder next times");
								return;	
							} else {
								state.life_point -= 1
							}
						}
						if choosed_word.chars().all(|x| state.guessed_characters.contains(&x)) {
							println!("You guessed the word, the answer is {choosed_word}");
							return;	
						}

					},
					None => {},	
				}
				continue;
			},
			Err(err) => {
				println!("{}", err);
				break;
			},	
		}
	}
}
