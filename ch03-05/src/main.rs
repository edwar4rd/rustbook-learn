use std::io;

fn c_to_f(num: f32) -> f32 {
    num / (100.0 - 0.0) * (212.0 - 32.0) + 32.0
}

fn nth_fib(n: u32) -> u32 {
    // if n == 0 {
    //     0
    // } else if n == 1 {
    //     1
    // } else {
    //     nth_fib(n - 1) + nth_fib(n - 2)
    // }

    if n == 0 {
        0
    } else {
        let mut this:u32 = 1;
        let mut last:u32 = 0;
        let mut nth:u32= 1;

        while nth != n {
            let next = this + last;

            last = this;
            this = next;
            nth += 1;
        }

        this
    }
}

fn main() {
    println!("Input a temperature to convert to Fahrenheit: ");

    let input: f32 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to readline!");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Cannot parse your input!");
                continue;
            }
        };
    };

    println!("You entered: {input}°C!");
    let converted = c_to_f(input);
    println!("That's {converted}°F!");

    println!("Input a integer to get a integer (wut): ");

    let input: u32 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to readline!");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Cannot parse your input!");
                continue;
            }
        };
    };

    println!("You entered: {input}!");
    let fib = nth_fib(input);
    println!("That's {fib}!");
}
