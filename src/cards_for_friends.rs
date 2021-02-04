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

fn highest_pow2_factor(n: usize) -> u32 {
    let mut pow2 = f64::log2(n as f64).floor() as u32;
    while n % 2usize.pow(pow2) != 0 {
        pow2 -= 1;
    }
    pow2
}

fn main() {
    let t = parse_input!(usize);
    for _ in 0..t {
        let (w, h, n) = parse_input!(usize, usize, usize);
        println!("{}", if 2usize.pow(highest_pow2_factor(w) + highest_pow2_factor(h)) >= n { "YES" } else { "NO" });
    }
}
