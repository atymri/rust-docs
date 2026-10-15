fn main() {
    let condition: bool = true;
    let num = if condition { 3 } else { 4 };

    println!("{}", num);

    // we have 3 kinds of loops:
    //  1.loop, 2.while, 3.for
    // loop: runs forever, until you stop it
    // while: runs forever, until the condition is false
    // for: runs for a set of turns

    // loop:
    // let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let mut i = 0;
    // let mut result: &str = "odd";

    // loop {
    //     result = if numbers[i] % 2 == 0 { "even" } else { "odd" };
    //     i = i + 1;
    //     if i == numbers.len() {
    //         break;
    //     }
    // }

    // print!(result);

    // // now we are gonna print some stars!
    // let i = 0;

    // loop{
    //     if i >= 10{
    //         break;
    //     }
    //     let mut j = 0;
    //     loop{
    //         if j >= i{
    //             break;
    //         }
    //         print!('*');
    //         j += 1;
    //     }
    //     println!();
    //     i += 1;;
    // }

    // let mut counter :f32 = 0.0;
    // let result = loop{
    //     counter += 1.0;

    //     if counter == 10.0 {
    //         break counter * 2.5; 
    //     }
    // };

    let nums: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    let mut i = 0;
    let mut sum = 0;

    let sum_res = loop{
        sum += nums[i];
        i += 1;

        if i != nums.len(){
            continue;
        }
        break sum;
    };

    let mut s_even = 0;
    let mut i = 0;

    let sum_of_even = loop{
        let current= nums[i];
        if current % 2 == 0{
            s_even += current;
        }
        i += 1;

        if i != nums.len(){
            continue;
        }
        break s_even;
    };

    println!("evens: {}", sum_of_even);
    println!("odds: {}", sum_res - sum_of_even);
    println!("total: {}", sum_res);

    let count = 10;
    for i in 0..count{
        for _j in 0..i{
            print!(" ");
        }
        for j in i..0{
            println!("*");
        }
        println!();
    }

}
