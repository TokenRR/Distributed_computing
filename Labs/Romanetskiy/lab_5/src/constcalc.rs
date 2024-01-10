use std::sync::{Arc, Mutex};
use rand::Rng;
use rayon::prelude::*;


pub fn picalc(n: i32) -> f64 {
    let w = 1.0 / n as f64;
    let f = |x: f64| 4.0/(1.0+x*x);
    let sum = Arc::new(Mutex::new(0.0));

    let _sum = sum.clone();
    (0..n).into_par_iter().for_each(|i| {
        let x = w * (i as f64 - 0.5);
        *_sum.lock().unwrap() += f(x);
    });

    let pi = w * (*sum.lock().unwrap());
    pi
}


pub fn mc_picalc(n: i32) -> f64 {
    let counter = Arc::new(Mutex::new(0 as i64));
    
    (0..n).into_par_iter().for_each(|_| {
        let x: f64 = rand::thread_rng().gen_range(-1.0..1.0);
        let y: f64 = rand::thread_rng().gen_range(-1.0..1.0);

        if x.powi(2) + y.powi(2) < 1.0 {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
        }
    });
    
    let counter = counter.lock().unwrap();
    let pi = 4.0 * (*counter as f64) / n as f64;  
    pi
}