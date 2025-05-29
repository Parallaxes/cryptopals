use std::env;

mod set01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    match query.as_str() {
        "2" => set01::challenge02::run().unwrap(),
        "1" => set01::challenge01::run().unwrap(),
        "3" => set01::challenge03::run().unwrap(),
        "4" => set01::challenge04::run().unwrap(),
        "5" => set01::challenge05::run().unwrap(),
        "6" => set01::challenge06::run().unwrap(),
        "7" => set01::challenge07::run().unwrap(),
        "8" => set01::challenge08::run().unwrap(),
        "0" => {
            set01::challenge02::run().unwrap();
            set01::challenge01::run().unwrap();
            set01::challenge03::run().unwrap();
            set01::challenge04::run().unwrap();
            set01::challenge05::run().unwrap();
            set01::challenge06::run().unwrap();
        }
        _ => panic!(),
    }
}
