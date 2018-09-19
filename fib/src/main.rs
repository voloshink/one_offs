use std::io;

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error reading in number");

        match buffer.trim().parse::<u32>() {
            Ok(i) => println!("{}", fib(i)),
            Err(_) => println!("Please enter a valid integer"),
        }
    }
}

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    let mut n1 = 0;
    let mut n2 = 1;
    let mut result = 0;

    for _ in 0..n - 1 {
        result = n1 + n2;
        n1 = n2;
        n2 = result;
    }

    result
}
