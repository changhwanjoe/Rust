//
//fn foo( v2 : Vec<i32> ) // v2 = v1
//fn foo( v2 : &Vec<i32> )
fn foo( v2 : &mut Vec<i32> ) //쓰기까지 하려면 이렇게 보내야됨
{
	println!("foo : {:?}", v2);
}

fn main()
{
	let mut v1 = vec![1,2,3];

//	foo(v1);
//	foo(v1.clone());
//	foo(&v1);
	foo(&mut v1); // 쓰기까지 하려면 이렇게 

	println!("{:?}", v1); 
}