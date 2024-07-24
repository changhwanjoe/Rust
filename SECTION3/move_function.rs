fn foo(x : i32, s : String)
{
}

fn main()
{
	let n = 10;
	let mut s = "ABC".to_string();
	
	foo(n, s); // s 는 copy 타입이라 이동될때 사라짐 
//	foo(n, s.clone());

//	s = "ABC".to_string(); 

	println!("{}", n);
	println!("{}", s);
}