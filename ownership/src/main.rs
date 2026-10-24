fn main() {
    // now we are learning about ownership and borrowing and etc
    
    // both stack and heap are part of memory that our code uses at the RUNTIME!
    // stack: puts the value in the order they come in, and removes them in reverse, lifo (last in first out)
    // heap: less organized than stack, allocators mark a part of it as used and then return an address to that part
    //       that address can be stored at stack, because it has a fixed size.

    // all primitive data types get stored at stack, because they have fixed size.
    // like, i8, u8, i32, u32, i64, u64, f32, f64, bool, char
    // but the values that their size might change during the runtime or compile time, get stored at heap.
    // String is one of them.

    let name = String::from("hello"); // this string gets stored at HEAP
    // let name2 = name; // here we will pass the 'name' refrence to name2, so both name and name2 are refrencing to the same content in the heap.

    // this is called shallow copy, just copying the refrence and not the value. 
    // but if we want to copy the name value from the heap and make a new one, we can use clone() method

    // name2 = name.clone();// now this is called a hard copy.

    // if we use the above variable as an argument for a method, that variable gets invalid at this scope,
    // becuase the method becomes its new owener, not this scope, and it gets removed with drop() function at the end of that functions scope, by rust
    str_length(name);

    // println!("{}", name); // error





    //let s1 = give_ownership();
    //let s2 = String::from("hello");
    //let s3 = take_and_give_back(s2);

    //println!("{}", s2); // now we can not use s2 here..

    let h1 = "hello";
    let (h2, l) = calculate_length(String::from(h1));
    println!("{}'s length is {}", h2, l);

    // by refrencing we can have the owenership of the variable, even if we pass it to some function
    // a refrence, is a pointer to a pointer(at least in this case)
    // this refrencing to something is called 'borrowing'
    let mut s1 = String::from("Hello");
    let len = calculate_refrence_length(&s1);
    println!("{}", len);

    // here in this function we will run into an error, because s1 is a refrence (its borrowed), 
    // we can not chage it
    // unless if we make it mutable
    change(&s1);

    // but this is okay (the variable itself hasa to be mutable tho)
    change_mut(&mut s1);
    println!("{}", s1);

    // NOTE: you can not have two mutable refrences to one value
    // this code below will not compile and it will fail:

    let mut org_val:String = String::from("Hello, World!");
    let ref1:&mut String = &mut org_val;
    // let ref2:&mut String = &mut org_val; // error 
    println!("ref1: {ref1}");
    
    // if we want to have anotehr refrence, we have to use the first one and then declare the second refrence.
    let ref2:&mut String = &mut org_val;
    println!("ref2: {ref2}");

    // this behavior prevents data race
    // data race happends when these three behavoirs acur:

        // two or more pointers access the same data at once
        // at least one of the pointers is being used to write to the data
        // there's no mechanism being used to synchronize access to the data

    // we have one more restriction, when you have immutable refrence, you can not have a mutable one.
    //let s = String::from("hello");
    //let r1 = &s; // no problem
    //let r2 = &s; // no problem

    // let r3 = &mut s; // BIG PROBLEM

    // Note that a reference’s scope starts from where it is introduced 
    // and continues through the last time that reference is used

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("val: {s} ref1: {r1} ref2: {r2}");

    // now because the scope of r1 and r2 is finished (they are used in println!)
    // we can have a mutable refrence 
    let r3 = &mut s;
    println!("mutable refrence: {r3}");


}

fn calculate_refrence_length(s: &String) -> usize{
    s.len()
} // here s goes out of scope. but becuase it does not have ownership of what  
  // it refers to, the String is not dropped.

fn change(s: &String){
  //  s.push_str(", World!") ----> ERROR
}

fn change_mut(s: &mut String){
    s.push_str(", World!");
}

fn str_length(str:String) -> usize{
    str.len()
}

fn give_ownership() -> String{
    let some_string:String = String::from("yours");
    some_string
}

fn take_and_give_back(a_string:String) -> String{
    a_string
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}
