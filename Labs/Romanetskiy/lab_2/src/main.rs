// cd /d/KPI/Distributed_computing/Labs/lab_2


use eframe::{epi, egui::{self, CtxRef}};
use rayon::{current_num_threads};
use std::process::Command;
use egui::vec2;

struct MyApp {
    //  дані та стан програми
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Ініціалізація стану
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Romanetskiy | Lab 2"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let button_size = vec2(ui.available_width(), 140.0);
            if ui.add_sized(button_size, egui::Button::new("Run Task 1")).clicked() {
                task_1()
            }
            if ui.add_sized(button_size, egui::Button::new("Run Task 2")).clicked() {
                task_2()
            }
        });
    }
}


fn main() {
    let app = MyApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(400.0, 300.0));
    eframe::run_native(Box::new(app), native_options);
}


fn task_1() {
    println!("\nTask 1:");
    let num_cores: usize = current_num_threads();
    println!("CPU cores number: {num_cores}");
    for i in 0..num_cores {
        let n = i + 1;
        println!("{n} Hello World !");
    }
}


fn task_2() {
    println!("\nTask 2:");
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
