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
/*
fn dist(w1: &str, w2: &str, a: usize, b: usize, dists: &mut std::collections::HashMap<(usize, usize), usize>) -> usize {
    if a == 0 || b == 0 {
        return a + b;
    }
    if w1[0..a] == w2[0..b] {
        return 0;
    }
    if let Some(ans) = dists.get(&(a, b)) {
        return *ans;
    }

    dists.insert((a, b), 0);
    let insert = dist(w1, w2, a - 1, b, dists) + 1;
    let delete = dist(w1, w2, a, b - 1, dists) + 1;
    let modify = dist(w1, w2, a - 1, b - 1, dists) + if w1[a-1..a] == w2[b-1..b] { 0 } else { 1 };

    dists.insert((a, b), usize::min(usize::min(insert, delete), modify));
    dists[&(a, b)]
}
*/
use std::collections::HashMap;

fn main() {
    let w1 = parse_input!();
    let w2 = parse_input!();
    let mut dists: HashMap<(usize, usize), usize> = HashMap::new();
    dists.insert((0, 0), 0);
    let mut a = 0;
    let mut b = 0;
    while a <= w1.len() {
        while b <= w2.len() {
            dists.insert((a, b), *[dists[&(a - 1, b)] + 1, dists[&(a, b - 1)] + 1, dists[&(a - 1, b - 1)] + if w1[a-1..a] == w2[b-1..b] { 0 } else { 1 }].iter().min().unwrap());
            b += 1;
        }
        a += 1;
    }
    println!("{}", dists[&(w1.len(), w2.len())]);
}
