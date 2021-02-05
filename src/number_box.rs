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

use std::convert::TryInto;

fn clamp(val: isize, min: isize, max: isize) -> isize {
    if val < min { min } else if val > max { max } else { val }
}

fn best_adj(array: &Vec<Vec<isize>>, row: usize, col: usize) -> (usize, usize) {
    let (mut r, mut c): (isize, isize) = (-1, -1);
    for i in (clamp(row as isize - 1, 0, array.len() as isize - 1) as usize)..=(clamp(row as isize + 1, 0, array.len() as isize - 1) as usize) {
        for j in (clamp(col as isize - 1, 0, array[row].len() as isize - 1) as usize)..=(clamp(col as isize + 1, 0, array[row].len() as isize - 1) as usize) {
            if (i == row) != (j == col) {
                if array[i][j] < *array.get(r.try_into().unwrap_or(usize::MAX)).unwrap_or(&vec![isize::MAX]).get(c.try_into().unwrap_or(usize::MAX)).unwrap_or(&isize::MAX) {
                    r = i as isize;
                    c = j as isize;
                }
            }
        }
    }
    (r as usize, c as usize)
}

fn main() {
    let t = parse_input!(usize);
    for _ in 0..t {
        let (rows, cols) = parse_input!(usize, usize);
        let mut a = Vec::new();
        for _ in 0..rows {
            a.push(parse_input!(vec isize));
        }
        for _ in 0..(rows * cols) {
            let (i1, j1, _) = a.iter().enumerate().fold((0, 0, a[0][0]), |(i, j, acc), (i1, v)| {
                let (j1, &val) = v.iter().enumerate().min_by(|(_, x), (_, y)| x.cmp(&y)).unwrap();
                if val < acc {
                    (i1, j1, val)
                } else {
                    (i, j, acc)
                }
            });
            let (i2, j2) = best_adj(&a, i1, j1);
            println!("[DBG] best={} best_adj={}", a[i1][j1], a[i2][j2]);
            if a[i1][j1] >= 0 && a[i2][j2] >= 0 {
                break;
            }
            a[i1][j1] *= -1;
            a[i2][j2] *= -1;
        }
        let sum = a.iter().fold(0, |acc, v| acc + v.iter().sum::<isize>());
        println!("{}", sum);
    }
}
