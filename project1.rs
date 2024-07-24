fn add(a : i32, b : i32) -> i32	
{
    return a + b;
}

fn subtract(a : i32, b : i32) -> i32	
{
	return a - b;
}

fn main() 
{
    let mut s = String::new();
	std::io::stdin().read_line(&mut s).unwrap();
	let vars:Vec<i32> = s.split_whitespace()
                            .map(|s| s.trim().parse().expect("err"))
                            .collect::<Vec<_>>();
//	let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    
  //  let n = input.next().unwrap();
	let var1:i32 = vars[0];
    let var2:i32 = vars[1];
    let var3:i32 = vars[2];
    if (var1 == 1)
	{
		println!("{}",add(var2,var3));
        
	}
    else if (var2 == 2)
    {
        println!("{}",subtract(var2,var3));
    }
}



    







