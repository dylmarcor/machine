pub mod calc;

// Function for calculating RPM - Rotations Per Minute
pub fn rpm(sfpm: f32, diameter: f32) -> f32 {
    3.82 * sfpm / diameter
}

// Function for calculating SFPM - Surface Feet Per Minute
pub fn sfpm() {
    
}

// Function for calculating machine time
pub fn time(ipr: f32, sfpm: f32, length: f32, diameter: f32) -> f32 {
    let rpm = rpm(sfpm, diameter);

    length / (rpm * ipr) 
}
