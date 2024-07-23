fn main()
{
	let v1 : () = ();
	let v2      = ();

//	println!("{}", v1);		// error 기본 출력은 안되고 디버그 출력만됨 
	println!("{:?}", v1);
	println!("{:?}", v2);
}

fn f1() // 반환값이 없다 void
{	
}

fn f3() -> () // 반환값이 없다 void
{
}