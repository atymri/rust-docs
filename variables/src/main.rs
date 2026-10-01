fn main() {
    println!("We are learning how to work with variables today.");
    // by default, all rust variables ar eimmutable, once assigned, YOU CAN NOT CHANGE THEM!
    // so when you run this code below, you will get an compiler error because you changed a variable.
    /*
    let x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");
    */

    // for making a mutable variable, you have to use the 'mut' keyword.
    // that means the variable is meant to be changed.
    let mut x = 12;
    println!("The value of x is {x}");
    x = 10;
    println!("The value of x is {x} now!");
    println!("now lets do some math on it");

    x += 10; // 20
    x *= 10; // 200
    x -= 10; // 190
    x /= 10; // 19

    println!("The value of x is now {x}"); // 19

    let mut author = "Amirmahdi";
    println!("Who is the author? {author}");

    author = "Teymouri";
    
    println!("his last name? {author}");

    println!("-----CONSTANTS-----");

    // note that variables are immutable for cuncurrency and other reasons by default,
    // but we can make them mutable when declaring them with the mut keyword

    // constants ARE NOT like that, they are meant to be immutable all the time
    // we declare constants with const keyword, and we have to specify the type of it too

    // const CONSTANT_NAME: TYPE = VALUE;

    const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;

    println!("{TWO_HOURS_IN_SECONDS}");

    //TWO_HOURS_IN_SECONDS = 12; // this will result in an error, obviously..
   
    /*
        error[E0070]: invalid left-hand side of assignment
    --> src/main.rs:46:26
    |
    46 |     TWO_HOURS_IN_SECONDS = 12; // this will result in an error, obviously..
    |     -------------------- ^
    |     |
    |     cannot assign to this expression

    For more information about this error, try `rustc --explain E0070`.
    error: could not compile `variables` (bin "variables") due to 1 previous error
    */

    println!("-----SHADOWING-----");

    // shadowing
    // for now we now that we can change a variables value if its made with mut
    // BUT with this approach, we can not assign diffrent valeus with different data types to the same variable
    // imagine that we have saved some spaces in a variable called spaces like this:
 
    //   let mut spaces = "     ";
    
    // now we want to replace the content of spaces to the count of spaces (here, with the NO 5)
 
    //   spaces = spaces.len();

    // now if you run this, you will get a compile tim error.
    /*
            error[E0308]: mismatched types
        --> src/main.rs:68:14
        |
        65 |     let mut spaces = "     ";
        |                      ------- expected due to this value
        ...
        68 |     spaces = spaces.len();
        |              ^^^^^^^^^^^^ expected `&str`, found `usize`

        For more information about this error, try `rustc --explain E0308`.
        error: could not compile `variables` (bin "variables") due to 1 previous error
    
    */

    // we can fix that with shadowing
    let spaces = "     ";
    let spaces = spaces.len();
    println!("count of spaces: {spaces}");

    let num = 5;
    let num = num * 2; // 10
    println!("num before scope: {num}");
    {
        // num wil be 100, only till the end of this scope.
        let num = num * 10; // 100
        println!("num inside scope: {num}");
    }
    println!("num after scope: {num}");

    // data types
    println!("-----DATA TYPES-----");
    // so far we learned that we need to specify the data type for the constant variables
    // now if we try to parse the value below, we will get compiler error

    // let number = "22".parse().expect("not a num");
    //println!("{number} is a number!");

    // becuase we need to specify the datatype in order for the parse() method to understand
    // what is the expected data type.

    // this is the error btw:
    /*
        error[E0284]: type annotations needed
        --> src/main.rs:112:9
            |
        112 |     let number = "22".parse().expect("not a num");
            |         ^^^^^^        ----- type must be known at this point
            |
            = note: cannot satisfy `<_ as FromStr>::Err == _`
        help: consider giving `number` an explicit type
            |
        112 |     let number: /* Type */ = "22".parse().expect("not a num");
            |               ++++++++++++

        For more information about this error, try `rustc --explain E0284`.
        error: could not compile `variables` (bin "variables") due to 1 previous error
    */

    // we can fix it by doing this:
    let num: u32 = "22".parse().expect("not a number!");
    println!("{num} is a number!");
}
