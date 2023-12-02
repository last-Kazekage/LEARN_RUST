use chrono::prelude::*;

fn main(){
    let now = Utc::now();
    let res = now.format("%Y-%m-%d %H:%M:%S");
    println!("date time: {}",res)
}
