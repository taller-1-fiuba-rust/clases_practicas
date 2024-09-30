use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct ThreadedApp {
    progress: Arc<Mutex<f32>>,
    is_running: Arc<Mutex<bool>>,
}

impl Default for ThreadedApp {
    fn default() -> Self {
        Self {
            progress: Arc::new(Mutex::new(0.0)),
            is_running: Arc::new(Mutex::new(false)),
        }
    }
}

impl eframe::App for ThreadedApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Threaded Task Example");

            let progress = *self.progress.lock().unwrap();
            let is_running_value = *self.is_running.lock().unwrap();

            ui.add(egui::ProgressBar::new(progress).show_percentage());

            let btn_label = if is_running_value {
                "Detener Tarea"
            } else {
                "Empezar Tarea"
            };

            if ui.button(btn_label).clicked() {
                let progress = Arc::clone(&self.progress);
                let is_running = Arc::clone(&self.is_running);

                if !is_running_value {
                    *is_running.lock().unwrap() = true;
                    thread::spawn(move || {
                        for i in 0..=100 {
                            if !*is_running.lock().unwrap() {
                                break;
                            }
                            *progress.lock().unwrap() = i as f32 / 100.0;
                            thread::sleep(Duration::from_millis(50));
                        }
                        *is_running.lock().unwrap() = false;
                    });
                } else {
                    *is_running.lock().unwrap() = false;
                }
            }

            if progress == 1.0 {
                ui.colored_label(egui::Color32::GOLD, "La tarea se ha finalizado.");
            }
        });

        // en cada nuevo frame se vuelve a generar la UI.
        if *self.is_running.lock().unwrap() {
            ctx.request_repaint();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Threaded Task Example",
        options,
        Box::new(|_cc| Box::new(ThreadedApp::default())),
    )
}
