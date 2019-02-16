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


    // Part 2
    // Data types


	// Length	Signed	Unsigned
	// 8-bit	i8	      u8
	// 16-bit	i16		  u16
	// 32-bit	i32		  u32
	// 64-bit	i64		  u64
	// 128-bit	i128	  u128
	// arch		isize	  usize


	    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    //Boolean Type

    let t = true;

    let f: bool = false; // with explicit type annotation

    //Chars type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //Tuple type
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    //Array type
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
}
