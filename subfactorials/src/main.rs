use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("unabled to read stdin");

        match input.trim().parse::<u32>() {
            Ok(i) => println!("{}", subfactorial(i)),
            Err(_) => println!("Please enter an integer"),
        }
    }
}

fn subfactorial(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 0,
        _ => (n - 1) * (subfactorial(n - 1) + subfactorial(n - 2)),
    }
}
