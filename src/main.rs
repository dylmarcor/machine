fn main() {
    let x: f32 = 90.0;
    let y: f32 = 1.5;
    println!("rpm is {:?}", rpm(x, y));
}

// Function for calculating RPM - Rotations Per Minute
fn rpm(sfpm: f32, diameter: f32) -> f32 {
    3.82 * sfpm / diameter
}

// Function for calculating SFPM - Surface Feet Per Minute
fn sfpm() {
    
}
