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

fn largest_sum(arr: Vec<isize>) -> usize {
    if arr.len() < 2 {
        return std::cmp::max(0, arr.iter().sum::<isize>()) as usize;
    }
    let mut largest = 0;
    for end in 0..=arr.len() {
        let temp = arr[0..end].iter().sum::<isize>();
        if temp > largest {
            largest = temp;
        }
    }
    largest as usize
}

fn main() {
    let n = parse_input!(usize);
    for _ in 0..n {
        let _ = parse_input!();
        let red = parse_input!(vec isize);
        let _ = parse_input!();
        let blue = parse_input!(vec isize);
        println!("{}", largest_sum(red) + largest_sum(blue))
    }
}
