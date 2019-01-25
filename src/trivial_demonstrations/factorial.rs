use std::io;
fn main()
{
	let mut fact:u128 = 1;
	println!("Enter a number");
	let mut n = String::new();
	io::stdin().read_line(&mut n).expect("Failed to read line");
	let n:u128= n.trim().parse().expect("Enter a number");
	for i in 1..n+1
	{
		fact = fact*i;
	}
	println!("Factorial of a given number is: {}",fact);
}