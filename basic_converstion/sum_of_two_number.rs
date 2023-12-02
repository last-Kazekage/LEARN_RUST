fn addition(x:f64,y:f64)->f64{
    x + y
}


fn main() {
    let x :f64 = 5.0;
    let y :f64 = 9.0;
    let output = addition(x,y);
    println!("addition of {} and {} is {}",x,y,output);

}
