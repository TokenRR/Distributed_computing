// cd /d/KPI/Distributed_computing/Labs/lab_3

use std::{sync::Arc, f64::consts::E};
use lab_3::integration::integral_reduction;
use eframe::{epi, egui::{self, CtxRef}};
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
        "Romanetskiy | Lab 3"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let button_size = vec2(ui.available_width(), 95.0);
            if ui.add_sized(button_size, egui::Button::new("Run Task 1")).clicked() {
                task_1()
            }
            if ui.add_sized(button_size, egui::Button::new("Run Task 2")).clicked() {
                task_2()
            }
            if ui.add_sized(button_size, egui::Button::new("Run Test")).clicked() {
                testing()
            }
        });
    }
}


fn task_1() {
    println!("\nTask 1:");
    let f = |x: f64| (E.powf(x) - 1.0) / (E.powf(x) + 1.0);
    let a = 1.0;
    let b = 2.0;
    let steps = 100;
    println!("f = (e^x-1) / (e^x+1), a={}, b={}, steps={}", a, b, steps);
    
    let af = Arc::new(f);

    let res = integral_reduction(&af, a, b, steps);
    println!("Result: {}", res);
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


fn main() {
    let app = MyApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(400.0, 300.0));
    eframe::run_native(Box::new(app), native_options);
}
