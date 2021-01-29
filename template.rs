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
    ( $( $ty:ty ),+ ) => {
        {
            let inp = input!();
            let mut inp = inp.split(' ');
            ($( inp.next().unwrap().parse::<$ty>().unwrap(), )+)
        }
    };
    ( vec $ty:ty ) => {
        {
            input!().split(' ').map(|v| v.parse::<$ty>().unwrap()).collect::<Vec<$ty>>()
        }
    }
}


fn main() {
    // Code goes here
}
