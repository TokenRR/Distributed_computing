use std::{sync::Arc};
use lab_3::integration::integral_reduction;
use std::process::Command;

fn main() {
    testing();
    task_1();
    task_2();
}

fn task_1() {
    println!("\n\n\nTask 1:");
    let f = |t: f64| (t + 1.0).ln();
    let a = 0.0;
    let b = 1.0;
    let steps = 10000;
    println!("f = ln(t + 1), a={}, b={}, steps={}", a, b, steps);
    
    let af = Arc::new(f);

    let res = integral_reduction(&af, a, b, steps);
    println!("Result: {}", res);
}

fn task_2() {
    println!("\n\n\nTask 2:");
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

fn testing() {
    println!("\nЗапускаємо cargo test");
    match Command::new("cargo").args(&["test"]).status() {
        Ok(status) => if status.success() {
            println!("cargo test виконано успішно");
        } else {
            eprintln!("cargo test завершилося з помилкою");
        },
        Err(e) => eprintln!("Помилка при запуску cargo test: {}", e),
    }
}
