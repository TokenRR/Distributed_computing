use rayon::{current_num_threads};
use std::process::Command;

fn main() {
    task_1();
    task_2();
}

fn task_1() {
    println!("\nTask 1:");
    let num_cores: usize = current_num_threads();
    println!("Ядер процесора: {}", num_cores);
    for i in 0..num_cores {
        let n = i + 1;
        println!("Вивід № {} Hello World!", n);
    }
}

fn task_2() {
    println!("\nTask 2:");
    match Command::new("cargo").args(&["bench"]).status() {
        Ok(status) => if status.success() {
            println!(" ");
        } else {
            eprintln!("cargo bench завершилося з помилкою");
        },
        Err(e) => eprintln!("Помилка при запуску cargo bench: {}", e),
    }
}
