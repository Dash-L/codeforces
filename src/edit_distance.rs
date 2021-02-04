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
    let word1 = parse_input!();
    let word2 = parse_input!();
    let mut ops = 0;
    let mut offset = 0;
    for (i, c) in word1.chars().enumerate() {
        if let Some(c2) = word2.chars().nth(i) {
            if c != c2 {
                ops += 1;
            }
        } else {
            // ...
        }
    }
    println!("{}", ops + (word1.len() as isize - word2.len() as isize).abs());
}
