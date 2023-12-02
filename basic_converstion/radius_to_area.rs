use std::io;

fn calculate_circle_area(radius: f64)->f64 {
    std::f64::consts::PI *radius.powi(2)
}

fn main(){
    println!("enter the radius of the circle: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    
    let radius: f64 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("invalid input.please enter a number");
            return;
        }
    };
    
    if radius< 0.0{
        println!("invalid input ,radius should be a non negative number");

    }else{
        let area = calculate_circle_area(radius);
        println!("the area of the circle with radius {} is {:.2}",radius,area);
    }
    
    
}
