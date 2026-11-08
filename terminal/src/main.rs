use eframe::egui;

struct terminal {
    title: String,
}

impl terminal {
    fn new() -> Self {
        Self {
            title: String::from("Terminal"),
        }
    }
}

impl eframe::App for terminal {

    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading(&self.title);

            ui.separator();

            ui.label("Welcome to the Rust Terminal Emulator");

            ui.label("Project 1 - Basic Window");

        });

    }
}

fn main() -> eframe::Result<()> {

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Rust Terminal Emulator",
        options,
        Box::new(|_cc| {
            Ok(Box::new(terminal::new()))
        }),
    )
}
