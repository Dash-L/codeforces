use std::io::stdin;

macro_rules! input {
    () => {
        {
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            s.trim().to_owned()
        }
    }
}

macro_rules! parse_input {
    ( $ty:ty ) => {
        input!().parse::<$ty>().unwrap()
    };

    ( $( $ty:ty ),+ ) => {
        {
            let inp = input!();
            let mut inp = inp.split(' ');
            ($( inp.next().unwrap().parse::<$ty>().unwrap(), )+)
        }
    };

    ( vec $ty:ty ) => {
        input!().split(' ').map(|v| v.parse::<$ty>().unwrap()).collect::<Vec<$ty>>()
    }
}


fn main() {
    let mut n = 0;
    let (_, k) = parse_input!(usize, usize);
    let ps = parse_input!(vec u32);
    for &p in ps.iter() {
        if p >= ps[k - 1] && p > 0 {
            n += 1;
        }
    }
    println!("{}", n);
}
