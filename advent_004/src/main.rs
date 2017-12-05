use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main()
{
	validate_duplicates();
	validate_anagrams();
}

fn validate_duplicates()
{
	let file = File::open("input.txt").expect("Error: File not found.");
	let reader = BufReader::new(&file);
	let mut sum: u32 = 0;
	for (_, line) in reader.lines().enumerate()
	{
		let text = line.unwrap();
		let words: Vec<String> = text.split(" ").map(|s| s.to_string()).collect();
		let mut valid: bool = true;
		for i in 0..words.len()
		{
			for j in 0..words.len()
			{
				if i != j
				{
					if words[i] == words[j]
					{
						valid = false;
						break;
					}
				}
			}
			if valid == false { break; }
		}
		if valid == true { sum += 1; }
	}
	println!("{}", sum);
}

fn validate_anagrams()
{
	let file = File::open("input.txt").expect("Error: File not found.");
	let reader = BufReader::new(&file);
	let mut sum: u32 = 0;
	for (_, line) in reader.lines().enumerate()
	{
		let text = line.unwrap();
		let words: Vec<String> = text.split(" ").map(|s| s.to_string()).collect();
		let mut valid: bool = true;
		for i in 0..words.len()
		{
			for j in 0..words.len()
			{
				if i != j
				{
					if is_anagram(words[i].to_string(), words[j].to_string())
					{
						valid = false;
						break;
					}
				}
			}
			if valid == false { break; }
		}
		if valid == true { sum += 1; }
	}
	println!("{}", sum);
}

fn is_anagram(a: String, b: String) -> bool
{
	let mut vec_a: Vec<_> = a.chars().collect();
	let mut vec_b: Vec<_> = b.chars().collect();
	for i in (0..vec_a.len()).rev()
	{
		for j in 0..vec_b.len()
		{
			if vec_a[i] == vec_b[j]
			{
				vec_b.remove(j);
				vec_a.pop();
				break;
			}
		}
	}
	return vec_a.len() == 0 && vec_b.len() == 0;
}