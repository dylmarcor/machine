use std::env;
use utils::*;

mod utils; 

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    println!("query is: {}", query);

    // test rpm
    let x: f32 = 90.0;
    let y: f32 = 1.5;
    println!("rpm is {:?}", rpm(x, y));

    // sfpm and time test
    let ipr: f32 = 0.005;
    let test_sfpm: f32 = 110.0;
    let length: f32 = 30.0;
    let diameter: f32 = 16.0;

    println!("sfpm is {:?}", sfpm(rpm(x, y), diameter));
    println!("time is {:?} minutes", time(ipr, test_sfpm, length, diameter));
}
