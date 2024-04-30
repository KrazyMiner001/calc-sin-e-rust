
use rug::{ops::Pow, Complete, Integer, Rational};

pub fn sinx(n: u32, x: Rational) -> Rational {
    let mut sinx = Rational::from(0);
    for i in 0..n {
        sinx += negative_one_to_n(i) * (x.clone().pow(2*i + 1) / Integer::factorial(2*i + 1).complete());
    }
    return sinx;
}

fn negative_one_to_n(n: u32) -> i8 {
    if n % 2 == 0 {
        return 1;
    } else {
        return -1;
    }
}