use std::io;

fn c_to_f(num :f32) -> f32 {
    num/(100.0-0.0)*(212.0-32.0)+32.0
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
}
