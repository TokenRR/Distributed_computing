// cd D:/KPI/Distributed_computing/Labs/lab_4

use lab_4::lineareq::gauss;
use ndarray::prelude::*;
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
        "Romanetskiy | Lab 4"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let button_size = vec2(ui.available_width(), 145.0);
            if ui.add_sized(button_size, egui::Button::new("Run Task")).clicked() {
                task()
            }
            if ui.add_sized(button_size, egui::Button::new("Run a test example")).clicked() {
                example()
            }
        });
    }
}


fn example() {
    let m = array![
        [-17., -82., 47., 56., 75., -34.],
        [-73., -40., -16., 34., 67., 21.],
        [83., 30., -55., -87., 72., 58.],
        [-96., -4., 60., 83., 25., -82.],
        [91., 16., -47., 39., -2., 25.],
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


fn main() {
    let app = MyApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(400.0, 300.0));
    eframe::run_native(Box::new(app), native_options);
}
