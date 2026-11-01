use eframe::egui;

struct MonitorApp {
    counter: i32,
}

impl Default for MonitorApp {
    fn default() -> Self {
        Self {
            counter: 0,
        }
    }
}

impl eframe::App for MonitorApp {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        ui.heading("Rust Hardware Monitor");

        ui.separator();

        ui.label("Welcome to our Hardware Monitor!");

        if ui.button("Increment Counter").clicked() {
            self.counter += 1;
        }

        ui.label(format!("Counter: {}", self.counter));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Hardware Monitor",
        options,
        Box::new(|_cc| Ok(Box::new(MonitorApp::default()))),
    )
}