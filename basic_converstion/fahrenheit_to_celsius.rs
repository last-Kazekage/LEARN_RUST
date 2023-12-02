use std::io;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64{
    (fahrenheit -32.0)*5.0/9.0
}

fn main(){
    println!("enter temperature in farenheit");
    
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    
    let fahrenheit: f64 = match input.trim().parse(){
        Ok(num) =>num,
        Err(_) => {
            println!("invalid input please enter a number");
            return;
        }
    };
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{} fahrenheit is {:.2} Celsius",fahrenheit,celsius)
}
