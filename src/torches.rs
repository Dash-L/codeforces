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


fn main() {
    let t = parse_input!(usize);
    for _ in 0..t {
        let (x, y, k) = parse_input!(usize, usize, usize);
        let n_sticks = (y + 1) * k - 1;
        let correction = if n_sticks % (x - 1) == 0 { 0 } else { 1 };
        println!("{}", n_sticks / (x - 1) + k + correction);
    }
}
