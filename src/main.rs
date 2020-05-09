use std::io::{self, Write};

fn lookup_helper(num: i64) -> String {
    let x: Option<&str> = match num {
        0 => Some("zero"),
        1 => Some("one"),
        2 => Some("two"), 
        3 => Some("three"), 
        4 => Some("four"), 
        5 => Some("five"),
        6 => Some("six"), 
        7 => Some("seven"), 
        8 => Some("eight"), 
        9 => Some("nine"),
        10 => Some("ten"), 
        11 => Some("eleven"), 
        12 => Some("twelve"), 
        13 => Some("thirteen"), 
        14 => Some("fourteen"), 
        15 => Some("fifteen"), 
        16 => Some("sixteen"), 
        17 => Some("seventeen"), 
        18 => Some("eighteen"),
        19 => Some("nineteen"), 
        20 => Some("twenty"), 
        30 => Some("thirty"),
        40 => Some("forty"),
        50 => Some("fifty"), 
        60 => Some("sixty"), 
        70 => Some("seventy"),
        80 => Some("eighty"),
        90 => Some("ninety"), 
        100 => Some("hundred"), 
        1000 => Some("thousand"),
        1000000 => Some("million"),
        1000000000 => Some("billion"),
        1000000000000 => Some("trillion"), 
        _ => None
    };

    return match x {
        Some(z) => z.to_string(),
        None => num_to_words(num),
    };
}

fn num_to_words(num: i64) -> String {
    if num == 0 {
        return lookup_helper(num);
    }
    let mut s = String::from("");

    let indices: [i64; 32] = [
        1000000000000, 1000000000, 1000000, 1000, 100,
        90, 80, 70, 60, 50, 40, 30, 20, 19, 18, 17,
        16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5,
        4, 3, 2, 1
    ];

    let mut temp = num;
    let mut acc = 0;
    while temp > 0 {
        let found = indices.iter().find(|&&idx| idx <= temp);
        let f = match found {
            None => break,
            Some(v) => {
                while (temp - v) >= 0 {
                    acc = acc + 1;
                    temp = temp - v;
                }
                v
            }
        };

        let count = if *f >= 100 { 
            lookup_helper(acc) + " "
        } else { 
            String::from("")
        };

        s = s + " " + &count.to_string() + &lookup_helper(*f);
        acc = 0
    }
    return s.trim().to_string()
}

fn test() {
    println!("10 is {}", num_to_words(10));
    println!("11 is {}", num_to_words(11));
    println!("35 is {}", num_to_words(35));
    println!("345 is {}", num_to_words(345));
    println!("305 is {}", num_to_words(305));
    println!("9305 is {}", num_to_words(9305));
    println!("109305 is {}", num_to_words(109305));
    println!("3109305 is {}", num_to_words(3109305));
    println!("123109305 is {}", num_to_words(123109305));
    println!("444123109305 is {}", num_to_words(444123109305));
    println!("2444123109305 is {}", num_to_words(2444123109305));
    println!("25444123109305 is {}", num_to_words(25444123109305));

}
fn main() {
    println!("i turn numbers into words");

    loop {

        print!("try me: ");
        io::stdout().flush().unwrap();

        let mut ss = String::new();
        match io::stdin().read_line(&mut ss) {
            Ok(bytes_read) => {
                ss = ss.trim().to_string();
                if bytes_read == 0 {
                    println!("okay bye (EOF)");
                    break
                }
                if ss == "quit" { 
                    println!("bye");
                    break
                }
                if ss == "test" {
                    test();
                    continue
                }
                let n = match ss.parse() {
                    Ok(z) => z,
                    Err(_) => {
                        println!("i don't understand");
                        continue;
                    }
                };
                println!("{} is {}", ss, num_to_words(n));
            },
            Err(_) => break,
        }
    }

}
