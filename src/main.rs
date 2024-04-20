use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let x: f32 = 90.0;
    let y: f32 = 1.5;
    println!("rpm is {:?}", rpm(x, y));

    // time test
    let ipr: f32 = 0.005;
    let sfpm: f32 = 110.0;
    let length: f32 = 30.0;
    let diameter: f32 = 16.0;

    println!("time is {:?} minutes", time(ipr, sfpm, length, diameter));
}

// Function for calculating RPM - Rotations Per Minute
fn rpm(sfpm: f32, diameter: f32) -> f32 {
    3.82 * sfpm / diameter
}

// Function for calculating SFPM - Surface Feet Per Minute
fn sfpm() {
    
}

// Function for calculating machine time
fn time(ipr: f32, sfpm: f32, length: f32, diameter: f32) -> f32 {
    let rpm = rpm(sfpm, diameter);

    length / (rpm * ipr) 
}
