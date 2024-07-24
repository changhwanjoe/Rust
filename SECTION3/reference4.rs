fn main()
{
	let mut n = 10;

//	let r1 : &i32 = &n; //immutable
//	*r1 = 20; // 에러 

	let r2 : &mut i32 = &mut n; // 읽고쓰기가능 
	*r2 = 20;


	println!("{}", n);
}

