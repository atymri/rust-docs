fn main() {
    // using ! at the end of the println means that println is a MACRO and not a FUNCTION!
    // macros are different from functions and they always end with !
    
    // this project is made manually and not with cargo
    // in large projects we use cargo, why?
    // 1. it builds the projects
    // 2. downloads the dependencies
    // 3. compiles the dependencies
    // basicly it does all the heavy liftings for us.
    println!("hello, world!");
    println!("im new to rust and im currently learning it! wish me luck..");
}
