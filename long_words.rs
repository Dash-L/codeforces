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
    let n = parse_input!(u32);
    for _ in 0..n {
        let (word,) = parse_input!(String);
        if word.len() > 10 {
            println!("{}{}{}", word.get(0..1).unwrap(), word.len() - 2, word.get((word.len() - 1)..(word.len())).unwrap());
        } else {
            println!("{}", word);
        }
    }
}
