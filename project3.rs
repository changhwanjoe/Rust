fn main() {
    let s1 = "B".to_string();
    let s2 = "A".to_string();
    let s3 = "X".to_string();
    let r = min(&s1,&s2,&s3);

    println!("{}", r);

}
fn min(alpha1:&str, alpha2:&str , alpha3:&str ) -> String 
{
    let mut strings = [alpha1, alpha2, alpha3];
    strings.sort();
    strings[0].to_string()
}
