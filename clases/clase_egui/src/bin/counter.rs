use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Counter App",
        options,
        // este tercer argumento es un puntero a una funcion que
        // dado un creation context devuelve una implementacion del trait eframe::App
        // es decir: Box<dyn FnOnce(&CreationContext<'_>) -> Box<dyn App>>
        Box::new(|_cc| Box::new(CounterApp::default())),
    )
}

struct CounterApp {
    // el estado interno de la aplicacion va a ser
    // unicamente el contador.
    count: i32,
}

// la implementacion de Default nos da el estado inicial de la app.
impl Default for CounterApp {
    fn default() -> Self {
        Self { count: 0 }
    }
}

// toda aplicacion requiere una implementacion del trait eframe::App.
// El metodo update define de que manera se actualizara la UI en cada frame.
// Aca es donde deberemos definir los componentes de la UI.
impl eframe::App for CounterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // creamos el panel central, que contendra los demas componentes de la UI.
        egui::CentralPanel::default().show(ctx, |ui| {
            // titulo
            ui.heading("Counter App");
            // especificamos el layour de la app, podria ser tambien vertical( .. ), vertical_centered, etc...
            ui.horizontal(|ui| {
                // agrego un boton, en caso de que sea clickeado, aumento en 1 el contador del estado.
                if ui.button("Increment").clicked() {
                    self.count += 1;
                }
                // agrego un label con el contador actual.
                ui.label(format!("Count: {}", self.count));

                if self.count % 2 == 0 && self.count != 0 {
                    ui.colored_label(egui::Color32::GOLD, "Se clickeo una cantidad par de veces.");
                }
            });
        });
    }
}
