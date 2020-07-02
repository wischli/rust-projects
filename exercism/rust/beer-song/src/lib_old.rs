fn on_the_wall(symbol: &str) -> String {
	" on the wall".to_owned() + symbol
}

fn bottles_of_beer(n: u32, start: bool) -> String {
	let bottles = match n {
		0 => {
			if start {
				"No more bottles".to_string()
			} else {
				"no more bottles".to_string()
			}
		}
		1 => n.to_string().to_owned() + " bottle",
		_ => n.to_string().to_owned() + " bottles",
	};
	bottles.to_owned() + " of beer"
}

fn take_verse(n: u32) -> String {
	let take = match n {
		1 => "it".to_string(),
		_ => "one".to_string(),
	};

	match n {
		0 => ".\nGo to the store and buy some more, ".to_string(),
		_ => ".\nTake ".to_owned() + &take + " down and pass it around, ",
	}
}

pub fn verse(n: u32) -> String {
	let n_minus_1 = match n {
		0 => 99,
		_ => n - 1,
	};
	bottles_of_beer(n, true)
		+ &on_the_wall(", ")
		+ &bottles_of_beer(n, false)
		+ &take_verse(n)
		+ &bottles_of_beer(n_minus_1, false)
		+ &on_the_wall(".\n")
}

pub fn sing(start: u32, end: u32) -> String {
	let mut song = String::new();
	for x in (end..start + 1).rev() {
		song.push_str(&verse(x));
		if x > end {
			song.push_str("\n");
		}
	}
	song
}
