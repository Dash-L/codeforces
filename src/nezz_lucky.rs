use std::io::stdin;

macro_rules! parse_input {
    ( ) => {
        {
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            s.trim().to_owned()
        }
    };
            
    ( $ty:ty ) => {
        parse_input!().parse::<$ty>().unwrap()
    };

    ( $( $ty:ty ),+ ) => {
        {
            let inp = parse_input!();
            let mut inp = inp.split(' ');
            ($( inp.next().unwrap().parse::<$ty>().unwrap(), )+)
        }
    };

    ( vec $ty:ty ) => {
        parse_input!().split(' ').map(|v| v.parse::<$ty>().unwrap()).collect::<Vec<$ty>>()
    }
}

fn is_lucky(num: usize, d: usize) -> bool {
    if num.to_string().contains(&d.to_string()) || num % d == 0 {
        true
    } else if num.to_string().len() == 1 {
        false
    } else {
        is_lucky(num - d, d)
    }
}

fn main() {
    let t = parse_input!(usize);
    for _ in 0..t {
        let (_, d) = parse_input!(usize, usize);
        let nums = parse_input!(vec usize);
        for num in nums {
            println!("{}", if is_lucky(num, d) { "YES" } else { "NO" });
        }
    }
}
