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

fn shortest_str(str1: &str, str2: &str) -> String {
    if str1.len() < str2.len() {
        str1.to_string()
    } else {
        str2.to_string()
    }
}

fn repeating_substr(str1: &str, str2: &str) -> String {
    let s_str = shortest_str(str1, str2);
    let mut current_substr = String::from("");
    for c in s_str.chars() {
        current_substr.push(c);
        if str1.replace(&current_substr, "").is_empty() && str2.replace(&current_substr, "").is_empty() {
            return current_substr;
        }
    }
    "".to_string()
}

fn count_substr(string: &str, substr: &str) -> usize {
    let mut count = 0;
    let mut string = String::from(string);
    while !string.is_empty() {
        string = string.replacen(substr, "", 1);
        count += 1;
    }
    count
}

fn lcm(n1: usize, n2: usize) -> usize {
   let mut n = n1;
   while n % n2 != 0 {
       n += n1;
   }
   n
}

fn main() {
    let n = parse_input!(usize);
    for _ in 0..n {
        let str1 = parse_input!();
        let str2 = parse_input!();
        let substr = repeating_substr(&str1, &str2);
        if substr.is_empty() {
            println!("-1");
            continue;
        }
        let count1 = count_substr(&str1, &substr);
        let count2 = count_substr(&str2, &substr);
        println!("{}", substr.repeat(lcm(count1, count2)));
    }
}
