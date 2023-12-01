fn main() {
	let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
	println!(
		"{}",
		include_str!("../../input.txt")
			.split("\n\n")
			.map(|e| {
				e.lines()
					.map(|w| {
						let mut v = Vec::with_capacity(w.len());
						let mut c = 0;
						for i in 0..w.len() {
							let s = &w[c..i + 1];
							let d: Vec<_> = s.chars().filter(|c| c.is_digit(10)).collect();
							if d.len() != 0 {
								v.push(d[0].to_string());
								c = i + 1;
							} else {
								for j in 0..words.len() {
									if s.contains(&words[j]) {
										v.push((j + 1).to_string());
										c = i - 1;
									}
								}
							}
						}

						match v.len() {
							0 => panic!("No digits found"),
							1 => format!("{}{}", v[0], v[0]),
							_ => format!("{}{}", v[0], v[v.len() - 1]),
						}.parse::<i32>().unwrap()
					})
					.sum::<i32>()
			})
			.sum::<i32>()
	);
}