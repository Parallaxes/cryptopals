use std::io::{self, Write};

mod challenges {
    pub mod set1 {
        pub mod challenge1;
        pub mod challenge2;
    }
}

fn main() {
    print!("Input Set #: ");
    io::stdout().flush().unwrap();
    let mut set_input = String::new();
    io::stdin().read_line(&mut set_input).unwrap();
    let set_input = set_input.trim().parse::<u8>().unwrap();

    print!("Input Challenge #: ");
    io::stdout().flush().unwrap();
    let mut challenge_input = String::new();
    io::stdin().read_line(&mut challenge_input).unwrap();
    let challenge_input = challenge_input.trim().parse::<u8>().unwrap();
    
    match set_input {
        1 => match challenge_input {
            1 => challenges::set1::challenge1::main(),
            2 => challenges::set1::challenge2::main(),
            _ => println!("Challenge not found"),
        },
        _ => println!("Set not found"),
    }

}
