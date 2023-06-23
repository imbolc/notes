pub mod a {
    pub mod series {
        pub mod of {
            #[derive(Debug)]
            pub enum TrafficLight {
                Red,
                Yellow,
                Green,
            }

            pub fn nested_modules() {
                println!("hi from module")
            }
        }
    }
}

// import a single function
use a::series::of::nested_modules;
// import enum values
use a::series::of::TrafficLight::{Green, Red};

fn main() {
    nested_modules();
    println!(
        "Go on {:?}, stop on {:?} and wait on {:?}",
        Green,
        Red,
        a::series::of::TrafficLight::Yellow
    );
}
