
fn print_stars(c:i32, s:char){
    for _i in 0..c{
        for _j in 0.._i{
            print!("{}", s);
        }
        println!();
    }
}


fn print(msg:&str){
    print!("{}", msg);
    io::stdout().flush().expect("failed to flush");
}

fn read() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("invalid input");

    input
}

// fn to_binary(num:i32) -> array::i32{
//     let powers_of_two: [i32;10];
//     for i in 1..10{
//         powers_of_two[i - 1] = 2
//     }
//     yield return 0;
// }

fn to_bin(mut n: i32) -> String {
    let mut result = String::new();

    let mut base:i32 = 0;

    while base < 30 && pow(2, base as i32) < n{
        base += 1;
    }

    for i in (0..=base).rev() {
        let p = pow(2, i as i32);
        if p <= n{
            n -= p;
            result.push('1');
        }else{
            result.push('0');
        }
    }

    result
}

fn pow(n: i32, p: i32) -> i32 {
    if p == 1 {
        return n;
    };
    if p == 0 {
        return 1;
    };

    let mut result: i32 = n;

    for _ in 1..p {
        result *= n;
    }

    result
}
