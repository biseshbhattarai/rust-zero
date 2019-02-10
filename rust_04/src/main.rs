fn main() {
    //Differences between mutables variables and shadowing..

    let mut x = 46;
	println!("{}", x);
	x = 56;
    println!("{}", x);
    x = "hello"
    println!("{}", x); // Must be assigned with the same data type else will not work...
    let y = "  ";
    let y = y.len(); // This code runs
    println!("{}", y);
}
