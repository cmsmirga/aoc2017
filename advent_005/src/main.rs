use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main()
{
    let file = File::open("input.txt").expect("Error: File not found.");
	let reader = BufReader::new(&file);
	let mut input: Vec<i32> = vec![];
	let mut sum: u32 = 0;
	for (_, line) in reader.lines().enumerate()
	{
		input.push(line.unwrap().parse::<i32>().unwrap());
	}
	let mut index: usize = 0;
	loop
	{
		sum += 1;
		if input[index] + (index as i32) < 0 || input[index] + (index as i32) > ((input.len() - 1) as i32) { break; }
		else
		{
			let temp = index;
			index = (input[index] + index as i32) as usize;
			//input[temp] += 1;
			if input[temp] >= 3 { input[temp] -= 1 }
			else { input[temp] += 1 }
		}
	}
	println!("{}", sum);
}
