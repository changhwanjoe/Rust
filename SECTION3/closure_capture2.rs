fn main()
{
	let mut v1 = 0;

//	let     f = || v1 = 20;
	let mut f = || v1 = 20; //mutable 하게 대여

	println!("{}", v1);
	
	f();

	println!("{}", v1);
}
