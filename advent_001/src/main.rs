use std::env;

fn main()
{
	let args: Vec<_> = env::args().collect();
	if args.len() > 1
	{
		let strings: Vec<String> = input_to_vec_string(args[1].to_string());
		let ints: Vec<u32> = input_to_vec_u32(strings);
		println!("{}", find_sum_of_trails(&ints));
		println!("{}", find_sum_of_half_circles(&ints));
	}
	else
	{
		println!("Syntax Error: You must enter an input string.");
	}
}

fn input_to_vec_string(input: String) -> Vec<String>
{
	let mut strings: Vec<String> = input.split("").map(|s| s.to_string()).collect();
	let length = strings.len() - 1;
	strings.remove(length);
	strings.remove(0);
	return strings;
}

fn input_to_vec_u32(input: Vec<String>) -> Vec<u32>
{
	let mut ints: Vec<u32> = vec![];
	for string in input
	{
		ints.push(string.parse::<u32>().unwrap());
	}
	return ints;
}

fn find_sum_of_trails(input: &Vec<u32>) -> u32
{
	let length = input.len();
	let mut sum: usize = 0;
	for i in 0..length
	{
		if (i != length - 1 && input[i] == input[i + 1])
		|| (i == length - 1 && input[i] == input[0])
		{
			sum += input[i] as usize;
		}
	}
	return sum as u32;
}

fn find_sum_of_half_circles(input: &Vec<u32>) -> u32
{
	let length = input.len();
	let mut sum: usize = 0;
	for i in 0..length
	{
		let mut halfway_point = i + length / 2;
		if halfway_point > length - 1 { halfway_point -= length }
		if input[i] == input[halfway_point] { sum += input[i] as usize }
	}
	return sum as u32;
}