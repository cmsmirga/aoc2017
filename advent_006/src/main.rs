use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main()
{
	let file = File::open("input.txt").expect("Error: File not found.");
	let reader = BufReader::new(&file);
	let text = reader.lines().last().unwrap().unwrap();
	let mut numbers: Vec<u32> = text.split("\t").map(|s| s.parse::<u32>().unwrap()).collect();
	println!("{}", count_till_repeating(&mut numbers));
	println!("{}", count_till_repeating(&mut numbers) - 1);
}

fn count_till_repeating(numbers: &mut Vec<u32>) -> u32
{
	let mut history: Vec<String> = vec![];
	let mut count = 0;
	loop
	{
		count += 1;
		let mut most_blocks = 0;
		for i in 0..numbers.len()
		{
			if numbers[i] > numbers[most_blocks] { most_blocks = i; }
		}
		let mut blocks = numbers[most_blocks];
		numbers[most_blocks] = 0;
		let mut index = most_blocks + 1;
		loop
		{
			if blocks == 0 { break; }
			if index == numbers.len() { index = 0 }
			numbers[index] += 1;
			blocks -= 1;
			index += 1;
		}
		let mut history_string: String = String::new();
		for i in 0..numbers.len()
		{
			history_string = history_string + " " + numbers[i].to_string().as_str();
		}
		println!("{}", history_string);
		if history.contains(&history_string) { break; }
		history.push(history_string);
	}
	return count;
}