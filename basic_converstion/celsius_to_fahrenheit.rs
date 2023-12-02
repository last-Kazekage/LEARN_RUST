use std::io;

fn celsius_to_fahrenheit(celsius: f64) -> f64{
    (1.8 * celsius)+32.0
}

fn main(){
    println!("enter temperature in celsius");
    
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    
    let celsius: f64 = match input.trim().parse(){
        Ok(num) =>num,
        Err(_) => {
            println!("invalid input please enter a number");
            return;
        }
    };
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{} celsius is {:.2} fahrenheit",celsius,fahrenheit)
}
