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
    let n = parse_input!(usize);
    let mut x = 0;
    for _ in 0..n {
        let line = input!();
        if line.contains("+") {
            x += 1;
        } else if line.contains("-") {
            x -= 1;
        }
    }
    println!("{}", x);
}
