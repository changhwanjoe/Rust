fn foo() -> i32
{
	return 10;
}

fn main()
{
	let n1 = 3;
	let n2 = (n1 + 2) * foo();

	foo();

	let n3 = {3};	// 블럭을 넣은 표현식
	let n4 = {3;};  // expression 이 아닌 expression statement

	println!("{:?}", n3); // 3
	println!("{:?}", n4); // Void ()
}