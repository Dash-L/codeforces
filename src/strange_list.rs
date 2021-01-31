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
    let n = parse_input!(usize);
    for _ in 0..n {
        let (_, x) = parse_input!(usize, usize);
        let mut a = parse_input!(vec usize);
        let mut i = 0;
        loop {
            if a[i] % x == 0 {
                for _ in 0..x {
                    a.push(a[i] / x);
                }
            } else {
                break;
            }
            i += 1;
        }
        println!("{}", a.iter().sum::<usize>());
    }
}
