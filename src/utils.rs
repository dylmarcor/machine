// Consts
const PI: f32 = core::f32::consts::PI;

// Function for calculating RPM - Rotations Per Minute
pub fn rpm(sfpm: f32, diameter: f32) -> f32 {
    3.82 * sfpm / diameter
}

// Function for calculating SFPM - Surface Feet Per Minute
pub fn sfpm(rpm: f32, diameter: f32) -> f32 {
    diameter * PI * (1.0/12.0) * rpm
}

// Function for calculating machine time
pub fn time(ipr: f32, sfpm: f32, length: f32, diameter: f32) -> f32 {
    let rpm = rpm(sfpm, diameter);

    length / (rpm * ipr) 
}
