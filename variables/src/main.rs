const MAX_POINTS_1: u32 = 100_000;

fn main() {
    println!("Hello, world!");

    // mut means mutable
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    // const is always immutable
    const MAX_POINTS_2: u32 = 100_000;

    // shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    // shadowing is different from mut
    let spaces = "   ";
    let spaces = spaces.len();

    // shadowing is useful when you want to change the type of a variable
    let guess = String::from("42");
    let guess: u32 = guess.trim().parse().expect("Not a number!");

    // Integer types
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize
    // arch is used to match the size of a pointer, which depends on the computer architecture
    // for example, 64-bit computer will have a 64-bit arch
    // arch usually used for indexing collections

    // Integer literals
    // Number literals	Example
    // Decimal	        98_222
    // Hex	            0xff
    // Octal	        0o77
    // Binary	        0b1111_0000
    // Byte (u8 only)	b'A'
    // type postfix:57u8, 1.0f64, 43i32

    // Interger overflow
    // let mut overflow: u8 = 255;
    // overflow += 1;
    // println!("overflow: {}", overflow);
    // In Debug mode:
    // thread 'main' panicked at src/main.rs:52:5:
    // attempt to add with overflow
    // In Release mode:
    // overflow: 0

    // Floating-Point Types
    // Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
    // default type is f64
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    // Integer division
    let quotient = 56 / 32; // 1
    println!("quotient: {}", quotient);
    // Float division
    let quotient = 56.7 / 32.3; // 1.755417956656347
    println!("quotient: {}", quotient);
    // Remainder
    let remainder = 43 % 5;

    // Boolean Type
    let t = true;

    // Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound Types
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
    // via index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{} {} {}", five_hundred, six_point_four, one);

    // Array
    // Array is fixed length
    // Array is allocated on stack
    // Every element of an array must have the same type
    // Array type is [T; N]
    // T is the type of each element
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // or
    let a = [1, 2, 3, 4, 5];
    // or
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    // via index
    let first = a[0];
    let second = a[1];
    // if index out of bound, will cause compile error
    // this code compiles failed:
    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);
    //  this code also compiles failed:
    // let index = [10,10,10];
    // let element = a[index[0]];
    // println!("The value of element is: {}", element);
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10'    

    // Functions
    // Function parameters: must declare the type of each parameter
    fn another_function(x: i32) {
        println!("The value of x is: {}", x);
    }
    // Function arguments
    another_function(5);
    // Function bodies contain statements and expressions
    // Statements are instructions that perform some action and do not return a value
    // Expressions evaluate to a resulting value
    // let y = 6; is a statement
    // let x = (let y = 6); is not allowed
    let y = {
        let x = 3;
        x + 1 // no semicolon
    }; // y = 4
    
    // Function return values
    // use -> to specify the type of the value returned
    fn five() -> i32 {
        5
    }
    println!("The value of five() is: {}", five());

    // Control Flow
    // if expression
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // if is an expression, so we can use it on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 }; // number: i32
    println!("The value of number is: {}", number);

    // loop
    // loop {
    //     println!("again!");
    // }
    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // for with range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
}    
