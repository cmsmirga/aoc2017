use std::env;

fn main()
{
	let args: Vec<_> = env::args().collect();
	let input: u32 = args[1].to_string().parse::<u32>().unwrap();
	println!("{}", manhattan_distance(input));
}

fn manhattan_distance(input: u32) -> i32
{
	let mut x: i32 = 0;
	let mut y: i32 = 0;
	let mut dx: i32 = 0;
	let mut dy: i32 = -1;
	for _ in 1..input
	{
		if (x == y) || ((x < 0) && (x == -y)) || ((x > 0) && (x == 1 - y))
		{
			let temp = dx;
			dx = -dy;
			dy = temp;
		}
		x += dx;
		y += dy;
	}
	return y.abs() + x.abs();
}

fn first_value_larger(input: u32) -> i32
{
	let mut x: i32 = 0;
	let mut y: i32 = 0;
	let mut dx: i32 = 0;
	let mut dy: i32 = -1;
	let mut spiral: Vec<u32> = vec![];
	spiral.push(1);
	for i in 1..input
	{
		if (x == y) || ((x < 0) && (x == -y)) || ((x > 0) && (x == 1 - y))
		{
			let temp = dx;
			dx = -dy;
			dy = temp;
		}
		x += dx;
		y += dy;
		spiral.push(get_next_number(x, y, &spiral));
	}
}

fn get_next_number(x: i32, y: i32, spiral: &Vec<u32>)
{
	if x > 0 
	return spiral[1] + 
}