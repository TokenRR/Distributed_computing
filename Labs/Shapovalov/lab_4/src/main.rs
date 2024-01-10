use lab_4::lineareq::gauss;
use ndarray::prelude::*;
use std::process::Command;

fn main() {
    example();
    task();
}

fn example() {
    let m = array![
        [2., 3., -1., 1.],
        [1., -1., 2., -3.],
        [3., 1., -2., 1.],
    ];
    println!("\nExample: \n{}", m);
    
    let m = gauss(m);
    println!("Result: {}", m);
}

fn task() {
    println!("\nTask:");
    println!("Запускаємо cargo bench");
    match Command::new("cargo").args(&["bench"]).status() {
        Ok(status) => if status.success() {
            println!("cargo bench виконано успішно");
        } else {
            eprintln!("cargo bench завершилося з помилкою");
        },
        Err(e) => eprintln!("Помилка при запуску cargo bench: {}", e),
    }
}
