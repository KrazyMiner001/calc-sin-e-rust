use std::thread;

use rug::{Complete, Integer, Rational};
use rug::ops::DivRounding;

fn p(a: &Integer, b: &Integer) -> Integer {
    if b.eq(&(a + 1u8).complete()) {
        Integer::ONE.clone()
    } else {
        let m = &(a + b).complete().div_floor(2);
        p(a, m) * q(m, b) + p(m, b)
    }
}

fn q(a: &Integer, b: &Integer) -> Integer {
    if b.eq(&(a + 1u8).complete()) {
        b.clone()
    } else {
        let m = &(a + b).complete().div_floor(2);
        q(a, m) * q(m, b)
    }
}

pub fn calc_e(n: u32) -> Rational {
    let (top, bottom) = thread::scope(|scope| {
        let top_thread = scope.spawn(|| p(&Integer::ZERO, &n.into()));
        let bottom_thread = scope.spawn(|| q(&Integer::ZERO, &n.into()));

        (top_thread.join().unwrap(), bottom_thread.join().unwrap())
    });

    let top: Rational = top.into();
    let bottom: Rational = bottom.into();

    1 + (top / bottom)
}