use std::io;
use std::env;
use utils::*;
use crate::help::help::*;

mod utils; 
mod help;

fn main() {
    let args: Vec<String> = env::args().collect();

    // initial test
    if args.len() > 1  {
        if args[1] == "help" {
            display_help()
        }

        // test rpm
        if args[1] == "rpm" {
            println!("rpm is good.");
            let rpm_sfpm: f32 = args[2].parse::<f32>().expect("unrecognized argument given");
            let rpm_diameter: f32 = args[3].parse::<f32>().expect("unrecognized argument given");
            println!("rpm is {:.2}", rpm(rpm_sfpm, rpm_diameter));
        }

        // test time
        if args[1] == "time" {
            let time_ipr: f32 = args[2].parse::<f32>().expect("unrecognized argument given");
            let time_sfpm: f32 = args[3].parse::<f32>().expect("unrecognized argument given");
            let time_length: f32 = args[4].parse::<f32>().expect("unrecognized argument given");
            let time_diameter: f32 = args[5].parse::<f32>().expect("unrecognized argument given");
            println!("time is {:.2?} minutes", time(time_ipr, time_sfpm, time_length, time_diameter));
        }
    } else {
        println!("no arguments given!"); 
    }

    // sfpm and time test
    // let ipr: f32 = 0.005;
    // let test_sfpm: f32 = 110.0;
    // let length: f32 = 30.0;
    // let diameter: f32 = 16.0;

    // println!("sfpm is {:?}", sfpm(rpm(rpm_sfpm, rpm_diameter), diameter));
    // println!("time is {:.2?} minutes", time(ipr, test_sfpm, length, diameter));
}
