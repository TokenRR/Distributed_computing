use rayon::prelude::*;
use ndarray::prelude::*;


pub fn gauss (mut m: Array<f64, Dim<[usize; 2]>>) -> Array<f64, Dim<[usize; 1]>> {
    let (nrows, ncols) = m.dim();

    // Прямий хід
    (0..nrows).into_iter().for_each(|i| {
        (0..ncols).rev().into_iter().for_each(|j| {
            m[[i,j]] /= m[[i,i]];            
        });

        let _m = m.clone();
        m
            .slice_mut(s![i+1..nrows, ..])
            .axis_iter_mut(Axis(0))
            .into_par_iter()
            .for_each(|mut row| {
                let scale = row[i];
                
                (i..ncols).into_iter().for_each(|k| {
                    row[k] -= scale * _m[[i,k]];
                });
            });
    });

    // Зворотній хід
    let mut xx = Array::<f64, _>::zeros(nrows);
    xx[nrows - 1] = m[[nrows - 1, nrows]];
    
    (0..nrows-1).rev().into_iter().for_each(|i| {
        xx[i] = m[[i, nrows]];

        (i+1..nrows).into_iter().for_each(|j| {
            xx[i] -= m[[i,j]] * xx[j];
        });
    });
    
    xx
}
