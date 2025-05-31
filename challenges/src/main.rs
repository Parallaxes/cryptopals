use std::env;

mod set01;
mod set02;

fn main() {
    let args: Vec<String> = env::args().collect();
    let set = &args[1];
    let challenge = &args[2];

    match set.as_str() {
        "1" => match challenge.as_str() {
            "1" => set01::challenge01::run().unwrap(),
            "2" => set01::challenge02::run().unwrap(),
            "3" => set01::challenge03::run().unwrap(),
            "4" => set01::challenge04::run().unwrap(),
            "5" => set01::challenge05::run().unwrap(),
            "6" => set01::challenge06::run().unwrap(),
            "7" => set01::challenge07::run().unwrap(),
            "8" => set01::challenge08::run().unwrap(),
            _ => {
                set01::challenge02::run().unwrap();
                set01::challenge01::run().unwrap();
                set01::challenge03::run().unwrap();
                set01::challenge04::run().unwrap();
                set01::challenge05::run().unwrap();
                set01::challenge06::run().unwrap();
                set01::challenge07::run().unwrap();
                set01::challenge08::run().unwrap();
            }
        },
        "2" => match challenge.as_str() {
            "9" => set02::challenge09::run().unwrap(),
            _ => panic!(),
        },
        _ => panic!(),
    }
}
