use std::io;

fn celius_to_fahrenheit(celius: f32) -> f32{
    return celius * 9.0/5.0 + 32.0;
}

fn main() {

    println!("Enter the celius temperature");

    let mut temperature = String::new();

    io::stdin().read_line(& mut temperature).expect("Failed to read line");

    let temperature: f32 = temperature.trim().parse().expect("Failed to parse str");

    println!("{} Celius is {} Fahrenheit", temperature, celius_to_fahrenheit(temperature));
}
