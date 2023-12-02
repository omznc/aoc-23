use std::collections::HashMap;

fn main() {
	println!("{}", include_str!("../../input.txt")
		.lines()
		.map(|game| {
			let game: Vec<_> = game.split(':').collect();
			let mut max_counts: HashMap<&str, i32> = HashMap::new();

			for group in game[1].split(';') {
				let mut group_counts: HashMap<&str, i32> = HashMap::new();

				for item in group.split(',') {
					let item: Vec<_> = item.trim().split_whitespace().collect();
					let count: i32 = item[0].parse().unwrap();
					let color = item[1];

					*group_counts.entry(color).or_insert(0) += count;
				}

				for (color, count) in group_counts {
					let max_count = max_counts.entry(color).or_insert(0);
					if count > *max_count {
						*max_count = count;
					}
				}
			}

			["red", "green", "blue"].iter().map(|color| max_counts.get(*color).unwrap_or(&0)).product::<i32>()
		})
		.sum::<i32>()
	);
}