fn main() {
    // Immutable and Mutable
    let mut x: i32 = 5;
    
    println!("Hello, world!");
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
    
    let sum = my_function(11, 2);
    println!("\n{}", sum);

    my_variables();
}

fn my_function(x: i32, y: i32) -> i32 {
    print!("x is: {}", x);
    print!("y is: {}", y);
    x + y
}

fn my_variables() {
    // Variables
    // 1. Integers

    let a: i32 = 98_222; // Decimals
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only)

    let f: u8 = 255; 

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);
    
    // 2. Floating-point Numbers

    let g: f64 = 2.3;
    let h: f32 = 1.8;
    println!("{}", g);
    println!("{}", h);

        // Addition
    let sum: i32 = 18 + 7;
    println!("{}", sum);

        // Subtraction 
    let difference: f32 = 23.06 - 18.07;
    println!("{}", difference);

        // multiplication
    let product: i32 = 4 * 10;
    println!("{}", product);

        // division
    let quotient: f64 = 18.7 / 10.4;
    println!("{}", quotient);

        // remainder
    let remainder: i32 = 43 % 54;
    println!("{}", remainder);

    // 3. Booleans
    let t: bool = true;
    let f: bool = false;

    println!("{}", t);
    println!("{}", f);
    

    // 4. Characters
    let a: char = 'a';
    let b: char = 'b';
    println!("{}", a);
    println!("{}", b);

    // Compound Types
    let tup: (&str, i64) = ("I feel like being Rusty!", 10_000);
    let (channel, sub_count) = tup;
    let channel_direct_assign: &str = tup.0;
    println!("{}", sub_count);
    println!("{}", channel);
    println!("{}", channel_direct_assign);

    let error_codes: [i32; 3] = [200, 404, 500];
    let not_found: i32 = error_codes[1];
    println!("{}", not_found);

    let byte = [0; 8];
    println!("{}", byte[1]);
}
