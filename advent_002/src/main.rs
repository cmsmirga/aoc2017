use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main()
{
	get_checksum();
	get_divisible_checksum();
}

fn get_checksum()
{
	let file = File::open("input.txt").expect("Error: File not found.");
	let reader = BufReader::new(&file);
	let mut sum: u32 = 0;
	for (_, line) in reader.lines().enumerate()
	{
		let text = line.unwrap();
		let numbers: Vec<u32> = text.split("\t").map(|s| s.parse::<u32>().unwrap()).collect();
		let mut max: u32 = numbers[0];
		let mut min: u32 = numbers[0];
		for i in 1..numbers.len()
		{
			if numbers[i] > max { max = numbers[i]; }
			if numbers[i] < min { min = numbers[i]; }
		}
		sum += max;
		sum -= min;
	}
	println!("{}", sum);
}

fn get_divisible_checksum()
{
	let file = File::open("input.txt").expect("Error: File not found.");
	let reader = BufReader::new(&file);
	let mut sum: u32 = 0;
	for (_, line) in reader.lines().enumerate()
	{
		let text = line.unwrap();
		let numbers: Vec<u32> = text.split("\t").map(|s| s.parse::<u32>().unwrap()).collect();
		let mut quotient: u32 = 0;
		for i in 0..numbers.len()
		{
			for j in 0..numbers.len()
			{
				if i != j
				{
					if numbers[i] % numbers[j] == 0 { quotient = numbers[i] / numbers[j]; }
				}
			}
		}
		sum += quotient;
	}
	println!("{}", sum);
}