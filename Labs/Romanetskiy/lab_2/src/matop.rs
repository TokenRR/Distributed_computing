use rayon::prelude::*;
use nalgebra::{DMatrix};


pub fn matmul_paralel(m1: &DMatrix<f64>, m2: &DMatrix<f64>) -> DMatrix<f64> {
    let m1shape = m1.shape();
    let m2shape = m2.shape();
    let nrows = m1shape.0;
    let ncols = m2shape.1;

    let iter: Vec<f64> = (0..m2shape.1).into_par_iter().flat_map(move |rj| {
        (0..m1shape.0).into_par_iter().map(move |li| {
            (0..m2shape.0)
                .zip(0..m1shape.1)
                .map(move |(ri, lj)| {
                    m1.index((li, lj)) * m2.index((ri, rj))
                })
                .sum()
        })
    })
    .collect();

    DMatrix::<f64>::from_iterator(nrows, ncols, iter)
}


pub fn matmul(m1: &DMatrix<f64>, m2: &DMatrix<f64>) -> DMatrix<f64> {
    let m1shape = m1.shape();
    let m2shape = m2.shape();
    let nrows = m1shape.0;
    let ncols = m2shape.1;

    let iter: Vec<f64> = (0..m2shape.1).flat_map(move |rj| {
        (0..m1shape.0).map(move |li| {
            (0..m2shape.0)
                .zip(0..m1shape.1)
                .map(move |(ri, lj)| {
                    m1.index((li, lj)) * m2.index((ri, rj))
                })
                .sum()
        })
    })
    .collect();
    
    DMatrix::<f64>::from_iterator(nrows, ncols, iter)
}


pub fn matelsum_paralel(m: &DMatrix<f64>) -> f64 {
    let shape = m.shape();
    
    (0..shape.0).into_par_iter().map(|i| {
        m.row(i).sum()
    }).reduce(|| 0.0, |a,b| a + b)
}


pub fn matelsum(m: &DMatrix<f64>) -> f64 {
    let shape = m.shape();

    (0..shape.0).into_iter().map(|i| {
        m.row(i).sum()
    }).reduce(|a,b| a + b).unwrap() 
}