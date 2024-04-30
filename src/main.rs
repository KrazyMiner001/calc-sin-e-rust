mod calc_e;
mod sinx;

use std::io::{self, Read};

use calc_e::calc_e;
use rug::Float;
use sinx::sinx;

fn main() {
    println!("Enter n value for e");
    let n_e_string= &mut "".to_string();
    io::stdin().read_line(n_e_string).expect("Could not read n for e");

    println!("Enter n value for sinx");
    let n_sin_string = &mut "".to_string();
    io::stdin().read_line(n_sin_string).expect("Could not read n for sinx");

    println!("Enter precision");
    let precision_string = &mut "".to_string();
    io::stdin().read_line(precision_string).expect("Could not read precision");

    let n_e = n_e_string.trim().parse::<u32>().unwrap();
    let n_sin = n_sin_string.trim().parse::<u32>().unwrap();
    let precision = precision_string.trim().parse::<u32>().unwrap();
    
    print!("{}", Float::with_val(precision, sinx(n_sin, calc_e(n_e))));
}
