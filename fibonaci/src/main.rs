use std::io;

fn fibonaci(mut n: u32) -> u32{
    let mut f0 = 0;
    let mut f1 = 1;

    if n == 0 {
        println!("0");
        return n;
    }

    if n == 1 {
        println!("1");
        return n;
    }

    let mut f2 = 0;

    println!("0\n1");

    while n - 2 > 0 {
        f2 = f0 + f1;
        f0 = f1;
        f1 = f2;
        n -= 1;
        println!("{}", f2);
    }
    return f2;
}

fn main() {

    println!("How many fibonaci do u want to print?");

    let mut counter = String::new();

    io::stdin().read_line(& mut counter).expect("Failed to read line");

    let counter: u32 = counter.trim().parse().expect("Failed to parse str");

    fibonaci(counter);
}
