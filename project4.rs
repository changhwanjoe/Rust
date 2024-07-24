const PI: f32 = std::f32::consts::PI;

struct Circle
{
	x   : f32, 
	y    : f32, 
	radius  : f32, 
}

impl Circle
{
	fn area( &self ) -> f32
	{
		PI*(self.radius)*(self.radius)
	}
	fn inflate( &mut self, r : f32 ) 
	{
		self.radius += r;
	}
    fn new( x : f32, y : f32, radius : f32) -> Self 
	{
		let rc = Circle{x, y, radius};
		rc
	}
}
impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // c2의 x값을 출력하도록 설정
        write!(f, "{} {} {}", self.x,self.y,self.radius)
    }
}


fn main() 
{
    let mut c1 = Circle{x:3.0,y:3.0,radius:5.0};
    let mut c2 = Circle::new(3.0,3.0,5.0);
    println!("{}",c2.area());
    c2.inflate(2.0);
    println!("{}",c2);
}
    
