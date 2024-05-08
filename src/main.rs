use eframe::{
    egui::{self, CentralPanel, Ui},
    run_native, App,
};

struct Machine {
    name: String,
}

impl Machine {
    fn new(name: String) -> Self {
        Self { name }
    }
}

// TODO: implement machines and networks as recursive portions of this simulation struct
struct Simulation {
    machines: Vec<Machine>,
}

impl Simulation {
    fn new() -> Self {
        Self { 
            machines: vec![Machine::new(String::from("harry"))]
        }
    }
}

impl App for Simulation {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            for m in &self.machines {
                ui.label(&m.name);
            }
        });
    }
}

pub fn main() {
    let app: Simulation = Simulation::new();
    let native_options = eframe::NativeOptions::default();
    run_native("sim builder", native_options, Box::new(|_cc| Box::new(app))).unwrap();
}
