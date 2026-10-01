use std::io;

fn main() {
    // we have TWO PRIMARY data types in rust

    // 1.scalar
    // 2.compunded

    // SCALAR:
    println!("-----SCALAR TYPES-----");
    // scalar types represent a single value, rust has 4 of them
    // integers, floating-point numbers, booleans, characters

    println!("-----INTEGERS-----");
    // we have 5 types of integers
    
    //   length     signed     unsigned
    //    8-bit :    i8          u8
    //   16-bit :    i16         u16
    //   32-bit :    i32         u32
    //   64-bit :    i64         u64
    //  128-bit :    i128        u128

    // note that you can not use negative numbers for unsigned forms.

    // number litrals
    // Decimal: 1_000
    // Hex: 0xff
    // Octal: 0o77
    // Binary: 0b111_000
    // Byte: b'A'

    println!("-----OVERFLOW-----");
    // each data type has its own range
    // range for signed: -(2^n-1) to (2^n-1) -1
    // e.g for i8:
    // -(2^7) to (2^7) - 1 => -128 , 127

    // range for unsigned: 0 to (2^n) - a
    // e.g for i8:
    // 0 to (2^8) - 1 => 0 , 255

    // let num: i8 = 128;
    // now if you check the code above you will get compiler error:
    /*
            error: literal out of range for `i8`
        --> src/main.rs:38:19
        |
        38 |     let num: i8 = 255;
        |                   ^^^
        |
        = note: the literal `255` does not fit into the type `i8` whose range is `-128..=127`
        = help: consider using the type `u8` instead
        = note: `#[deny(overflowing_literals)]` on by default

        error: could not compile `datatypes` (bin "datatypes") due to 1 previous error
    */
    // in release mode, if the value is above the range, it will be 1 and if its below, it will be 0
    // so, the program wouldnt panic.

    println!("-----FLOATING POINT NUMS-----");
    // rust has 2 primary floating point types
    // f32 and f64
    // default is f64, both of them are signed.

    let _x = 12.2;       // f64;
    let _y : f32 = 12.2; // f32

    println!("-----MATHEMATICAL OPERATIONS-----");
    // addition
    let sum = 3300 + 1;

    // subtraction
    let diff = 12.2 - 4.3;

    // multipication
    // note: you can not multiply an integer with a float
    // let product = 14.4 * 2;
    let product = 14.4 * 0.2;

    // division
    // Integer division truncates toward zero to the nearest integer
    let quotient = 56.7 / 3.22;
    let truncated = -5 / 3; // res is -1

    // remainder
    let remainder = 43 % 5;

    println!("sum: {sum}\ndiff: {diff}\nproduct: {product}\ndivision: {quotient}\ntruncated: {truncated}\nremainder: {remainder}");

    println!("-----BOOLEAN TYPE-----");
    let _t = true;
    let _f: bool = false;

    let _check: bool = 12 / 1 == 13;
    println!("{_check}");

    println!("-----CHARACTER TYPE-----");
    // rusts char type is the languages most primitive alphabetic type.
    let _c = 'z';
    let _z = 'Z';
    let _crab_emoji = '🦀';
    // we store represent chars using ' instead of " just like csharp.

    // COMPOUND
    println!("-----COMPOUND TYPES-----");
    // compound types can group multiple values into one type
    // rust has 2 primitive compound types
    // 1.TUPLES
    // 2.ARRAYS

    println!("TUPLES");
    // tuple is a general way for grouping together a number of values with a variety of types into on compound type.
    // TUPLES HAVE A FIXED LENGTH, once declared, they can not shrink or grow in size.

    // we create a tuple with comma seperated list of values inside a parentheses.
    // each position in a tuple has a type and the type of the different values in the tuple don't have to be same.

    let tup: (f32, i32, u64) = (12.2, 13, 14);
    // the tup variable binds the entire tuple because a tuple is considered as a single compound type.
    // to get individual values from a tuple we can use pattern matching.

    let (_x, y, _z) = tup;
    println!("the vlaue of y is {y}");

    // we can get values with indexes too, BUT, its different 
    // we use the tuple name followed by a . and the index
    // like this: tup.0

    let x = (1, 2.2, 3); // types are u32, f64, u32

    let first = x.0;
    let second = x.1;
    let third = x.2;
    
    println!("first: {first}\nsecond: {second}\nthird: {third}");

    println!("ARRAYS");
    // they have fixed size, and the elements must be the same type.

    let arr: [f32; 5] = [0.2, 1.2, 3.6, 7.0, 15.0];
    //println!("{}", arr.0); // you can not use the indexes like tuples
    println!("{}", arr[0]);

    // arrays are fixed size, tuples are also fixed, just vectors are flexibl
    // arrays stay on the stack while the vectors stay on the heap, so thats why they are flexible.

    let months = ["January", "February", "March", "April",
                             "May", "June", "July", "August", "September",
                             "October", "November", "December"];

   let months_in_persian = ["فروردین", "اردیبهشت", "خرداد", "تیر",
                                       "مرداد", "شهریور", "مهر", "آبان",
                                       "آذر", "دی", "بهمن", "اسفند", ];

   println!("{} {}", months[3], months_in_persian[3]);

   // you can make an array with fixed value and multiple counts like this:
   let _all_threes: [u32; 10] = [3; 10];
   // equals to
   let _all_threes: [u32; 10] = [3, 3, 3, 3, 3, 3, 3, 3, 3, 3];


   let a: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
   println!("please enter an array index: ");
   
   let mut index = String::new();
   
   io::stdin()
        .read_line(&mut index)
        .expect("invalid input");

    let index: usize = index
            .trim()
            .parse()
            .expect("index entered is not a number");

    let element = a[index];
    println!("the value of the element index {index} is : {element}");

}
