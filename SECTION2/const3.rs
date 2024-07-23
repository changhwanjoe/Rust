fn main()
{
	let   size : usize = 10;
	const SIZE : usize = 10;

	let arr1 : [i32;10];	// ok i32타입 10개짜리 배열
//	let arr2 : [i32;size]; 	// error . immutable 상수는 안됨 
	let arr3 : [i32;SIZE]; 	// ok
}