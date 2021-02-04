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
    let w1 = parse_input!();
    let w2 = parse_input!();
    let n = w1.len();
    let m = w2.len();
    let mut dists: Vec<Vec<usize>> = (0..=n).map(|_| [0].repeat(m + 1)).collect();
    let mut a = 0;
    let mut b;
    while a <= n {
        b = 0;
        while b <= m {
            if a == 0 || b == 0 {
                dists[a][b] = a + b;
                b += 1;
                continue;
            }
            dists[a][b] = *[
                dists[a - 1][b] + 1,
                dists[a][b - 1] + 1,
                dists[a - 1][b - 1] + if w1.chars().nth(a - 1).unwrap() == w2.chars().nth(b - 1).unwrap() { 0 } else { 1 },
            ].iter().min().unwrap();
            b += 1;
        }
        a += 1;
    }
    println!("{}", dists[n][m]);
}
