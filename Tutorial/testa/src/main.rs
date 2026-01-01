use std::io;
fn main(){
    println!("Ge mig en lista av number separerade med mellansteg: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input kunde inte läsas");

    let num: i32 = input.trim().parse().expect("Input kunde inte hanteras som nummmer");
    match (num % 3 == 0, num % 5 == 0) {
        (true, true) => println!("Fizzbuzz"),
        (true, false) => println!("Fizz"),
        (false, true) => println!("Buzz"),
        (false, false) => println!("{}", num),
    }


}