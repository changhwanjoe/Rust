fn next_num() -> i32
{
	static mut CNT : i32 = 0; // 함수 호출이 종료 되어도 파괴되지 않음 
	
	unsafe { CNT = CNT + 1;}

	let n = unsafe{CNT};

//	static CNT : i32 = 0;
	return n;
}

fn main() // 호출할때마다 1이 증가하는 함수 
{
	println!("{}", next_num()); // 1 
	println!("{}", next_num()); // 2
}