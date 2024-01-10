// cd /d/KPI/Distributed_computing/Labs/lab_1

#[macro_use]
extern crate lazy_static;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType, Box, Orientation};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use thread_priority::*;
use std::os::windows::io::AsRawHandle;
use winapi::ctypes::c_void as winapi_c_void;

lazy_static! {
    static ref PRIORITY_MIN: ThreadPriority = ThreadPriority::Os(WinAPIThreadPriority::Lowest.into());
    static ref PRIORITY_MAX: ThreadPriority = ThreadPriority::Os(WinAPIThreadPriority::Highest.into());
}

fn increment(refr: Arc<Mutex<i32>>) {
    loop {
        let mut num = refr.lock().unwrap();
        *num += 1;
        println!("Incremented: {}", num);
        thread::sleep(Duration::from_millis(100));
    }
}

fn decrement(refr: Arc<Mutex<i32>>) {
    loop {
        let mut num = refr.lock().unwrap();
        *num -= 1;
        println!("Decremented: {}", num);
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let counter = Arc::new(Mutex::new(0));

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Romanetskiy | Thread Priority Control");
    window.set_default_size(300, 200);

    let vbox = Box::new(Orientation::Vertical, 0);

    let btn_inc_min = Button::with_label("Increment Min");
    let btn_inc_max = Button::with_label("Increment Max");
    let btn_dec_min = Button::with_label("Decrement Min");
    let btn_dec_max = Button::with_label("Decrement Max");

    vbox.pack_start(&btn_inc_min, true, true, 0);
    vbox.pack_start(&btn_inc_max, true, true, 0);
    vbox.pack_start(&btn_dec_min, true, true, 0);
    vbox.pack_start(&btn_dec_max, true, true, 0);

    window.add(&vbox);

    let counter_clone_inc = Arc::clone(&counter);
    let inc_thread = thread::spawn(move || {
        increment(counter_clone_inc);
    });

    let counter_clone_dec = Arc::clone(&counter);
    let dec_thread = thread::spawn(move || {
        decrement(counter_clone_dec);
    });

    let inc_handle = inc_thread.as_raw_handle() as *mut winapi_c_void;
    let dec_handle = dec_thread.as_raw_handle() as *mut winapi_c_void;

    btn_inc_min.connect_clicked(move |_| {
        set_thread_priority(inc_handle, *PRIORITY_MIN).expect("Failed to set thread priority");
    });

    btn_inc_max.connect_clicked(move |_| {
        set_thread_priority(inc_handle, *PRIORITY_MAX).expect("Failed to set thread priority");
    });

    btn_dec_min.connect_clicked(move |_| {
        set_thread_priority(dec_handle, *PRIORITY_MIN).expect("Failed to set thread priority");
    });

    btn_dec_max.connect_clicked(move |_| {
        set_thread_priority(dec_handle, *PRIORITY_MAX).expect("Failed to set thread priority");
    });

    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
