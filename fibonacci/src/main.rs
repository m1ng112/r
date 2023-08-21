use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Your choice is {}", input);

    let num: u32 = input.trim().parse().expect("Please type a number!");

    if num <= 0 {
        println!("You typed zero!");
        return;
    } else if num == 1 {
        println!("You typed one!");
        return;
    }

    let (_, b) = fibonacci(num);

    println!("The {}th fibonacci number is {}", num, b);
}

fn fibonacci(n: u32) -> (u32, u32) {
    if n == 1 {
        return (0, 1);
    }

    let (a, b) = fibonacci(n - 1);

    println!("{}th fibonacci number is {}", n, (a + b));

    (b, a + b)
}
