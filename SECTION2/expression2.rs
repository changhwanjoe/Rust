fn f1() -> i32 
{
//	 return 10;
	 10 		// 위와 동일
//	 10;
}

fn main()
{
	let score = 30;

	let total = { let report = 30; 
				  score + report }; // 블럭도 표현식 . 정답 60이 나옴 . ; 붙이면 void 됨 
	
	println!("{:?}", total);

	println!("{:?}", f1() );
}


