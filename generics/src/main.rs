//  cargo add num-traits
use num_traits::{Float, ToPrimitive};
fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

// 1. we can pass in BOTH f32 and f 64
fn solve_both<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn solve_diff<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn solve_primitive<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b: f32 = 4.0;
    let c: u8 = 4;

    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b as f64;
    println!("solve {}", solve(a_f64, b_f64));

    println!("solve_both {}", solve_both::<f32>(a, b));
    println!("solve_both {}", solve_both::<f64>(a_f64, b_f64));
    println!("solve_diff {}", solve_diff(a, b_f64));
    println!("solve_primitive {}", solve_primitive(a, c));
}
