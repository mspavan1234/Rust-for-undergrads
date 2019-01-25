use std::io;
fn main()
{
	println!("Enter a number:");
	let mut n = String::new();
	io::stdin().read_line(&mut n).expect("Falied to read line");
	let n:u128 = n.trim().parse().expect("Enter a number");
	print!("Prime numbers are:");
	for i in 2..n+1
	{
		let mut count:i8 = 0;
		for j  in 1..i+1
		{
			if i%j==0
			{
				count = count+1;
			}
		}
		if count==2
		{
			print!(" {}",i);
		}
	}
}