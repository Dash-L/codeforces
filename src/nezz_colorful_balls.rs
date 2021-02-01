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
        let _ = parse_input!();
        let balls = parse_input!(vec usize);
        let mut max_dups = 0;
        for ball in &balls {
            let n_dups = balls.iter().filter(|&n| n == ball).count();
            if n_dups > max_dups {
                max_dups = n_dups;
            }
        }
        println!("{}", max_dups);
    }
}
