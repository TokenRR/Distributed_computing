// cd D:/KPI/Distributed_computing/Labs/lab_5


use egui::vec2;
use std::time::Instant;
use eframe::{epi, egui::{self, CtxRef}};
use lab_5::constcalc::{picalc, mc_picalc};
use rayon::ThreadPoolBuilder;


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
        "Romanetskiy | Lab 5"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let button_size = vec2(ui.available_width(), 140.0);
            if ui.add_sized(button_size, egui::Button::new("Run integral method")).clicked() {
                task_intergral()
            }
            if ui.add_sized(button_size, egui::Button::new("Run Monte-carlo method")).clicked() {
                task_monte_carlo()
            }
        });
    }
}


fn format_dots(n: i32) -> String {
    let mut dots = n.to_string();
    let len = dots.len();
    if len > 3 {
        let e = len - 1;
        dots = format!("1e{}", e);
    }
    dots
}


fn task_intergral() {
    println!("\nIntegral method:");
    for &threads in [1, 2, 4, 8].iter() {
        let pool = ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        for &n in [1e3 as i32, 1e4 as i32, 1e5 as i32, 1e6 as i32, 1e7 as i32].iter() {
            let start = Instant::now();
            pool.install(|| {
                let pi = picalc(n);
                let duration = start.elapsed();
                println!("  threads = {:<2} | time = {:<10} | intervals = {:<2} | result = {:.10}",
                        threads, format!("{:.4?}", duration), format_dots(n), pi);
            });
        }
        println!();
    }
}


fn task_monte_carlo() {
    println!("\n\n\nMonte-Carlo method:");
    for &threads in [1, 2, 4, 8].iter() {
        let pool = ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        for &n in [1e3 as i32, 1e4 as i32, 1e5 as i32, 1e6 as i32, 1e7 as i32].iter() {
            let start = Instant::now();
            pool.install(|| {
                let pi = mc_picalc(n);
                let duration = start.elapsed();
                println!("  threads = {:<2} | time = {:<10} | dots = {:<2} | result = {:.10}",
                        threads, format!("{:.4?}", duration), format_dots(n), pi);
            });
        }
        println!();
    }
}


fn main() {
    let app = MyApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(400.0, 300.0));
    eframe::run_native(Box::new(app), native_options);
}
