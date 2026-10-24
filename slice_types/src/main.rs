fn main() {
    // requirement is simple, we need a function that returns the first word in a string
    // if we give him the string below:
    println!("Hello, world!");
    // we should get "Hello"
    // or at least, we should know that where the first word ends, before the whitespace

    let mut word = String::from("Hello, World!");
    let first_index = first_word(&word);

    println!("first word of {} ends at {}", word, first_index);
    println!("first word of {} is {}", word, word[..first_index].to_string());

    // yeah for sure this function works, and it works well,
    // BUT what if we change/remove the 'word' completely? the 'first_index' doesn't change in that case
    word.clear();
    println!("now the word is gone, but we still got the last index of its first word: {}", first_index);
    // and thats bad, TOO BAD!
    // slices are the solution, reference slices to be accurate.

    let mut word = String::from("Rust is one of the best toys i have ever played with.");
    let first_word = first_word_but_better(&word);
    println!("first word of:\n{}\n is: \"{}\"", word, first_word);

    // now if we make some changes in the 'word' variable or remove it completely, 
    // we will lose the 'first_word'...

    // word.clear(); //ERROR! 
    // the code above will generate the error below:
    // cannot borrow `word` as mutable because it is also borrowed as immutable 
    println!("{}", first_word);
    // why the hell that happened? 

    // because we are returning a reference to a part of that original string,
    // so, that means if we make the original string as mutable, the reference might become invalid in future.
    // yeah compiler yells at us because of this.








    let words = "one two three four five six seven eight nine ten"; // 10 words

    let first_w = n_word(&words, 1);
    let second_w = n_word(&words, 2);
    let third_w = n_word(&words, 3);
    let seven_w = n_word(&words, 7);

    println!("first: {}", first_w);
    println!("second: {}", second_w);
    println!("third: {}", third_w);
    println!("seven: {}", seven_w);

}

fn first_word(s: &String) -> usize{
    let s_bytes = s.as_bytes();
    
    for (i, &item) in s_bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    s.len()
}

fn first_word_but_better(s: &str) -> &str{
    let s_bytes = s.as_bytes();
    
    for (i, &item) in s_bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

// just messing around
fn n_word(s: &str, n:usize) -> &str{

    let s_bytes = s.trim().as_bytes();
    let mut indexes:Vec<usize> = vec![];

    for (i, &item) in s_bytes.iter().enumerate(){
        if item == b' '{
            indexes.push(i);
        }
    }
    let start:usize;
    let end:usize;
    if n == 1{
        start = 0;
        end = indexes[0];
    }else{
        start = indexes[n - 2];
        end = indexes[n - 1];
    }

    return &s[start..end].trim();
}
