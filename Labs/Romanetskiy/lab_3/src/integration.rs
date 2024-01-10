use std::sync::{Arc};
use std::marker::{Sync, Send};
use rayon::prelude::*;


pub fn integral_reduction(f: &Arc<impl Fn(f64) -> f64 + Sync + Send>, a: f64, b: f64, steps: i32) -> f64 {
    if steps % 2 != 0 {
        panic!("The number of steps must be even for the Simpson method");
    }

    let dx = (b - a) / steps as f64;
    let mut sum = f(a) + f(b);

    sum += (1..steps).into_par_iter().map(|i| {
        let x = a + i as f64 * dx;
        let coeff = if i % 2 == 0 { 2.0 } else { 4.0 };
        coeff * f(x)
    }).sum::<f64>();

    sum * dx / 3.0
}


#[cfg(test)]
mod tests {
    use super::integral_reduction; 
    use std::sync::{Arc};
    use std::f64::{consts::E};

    #[test]
    fn test_example_sample() {
        println!("Task 1:");
        let f = |x: f64| (E.powf(x) - 1.0) / (E.powf(x) + 1.0); 
        let a = 1.0; 
        let b = 2.0; 
        let steps = 100; 
        
        let af = Arc::new(f);

        let res = integral_reduction(&af, a, b, steps);
        assert!(res.is_finite(), "Result should be finite and not INFINITY");
    }
}
