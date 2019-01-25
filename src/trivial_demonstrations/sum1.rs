use std::io;
fn main()
{
	println!("Enter a number: ");
	let mut n = String::new();
	io::stdin().read_line(&mut n).expect("Failed to read linr");
	let n:u128 = n.trim().parse().expect("Enter a number");
	let mut sum:u128 = 0;
	for i in 1..n+1
	{
		sum=sum+i;
	}
	println!("Sum of n natural numbers: {}",sum);
}