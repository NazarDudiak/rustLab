use eframe::egui;

struct CalculatorApp {
    memory: f64,
    input: String,
    message: String,
}

impl CalculatorApp {
    fn new() -> Self {
        CalculatorApp {
            memory: 0.0,
            input: String::new(),
            message: String::from("Welcome to Calculator!"),
        }
    }

    fn add(&mut self) {
        if let Ok(value) = self.input.parse::<f64>() {
            self.memory += value;
            self.message = format!("Result: {:.2}", self.memory);
        } else {
            self.message = format!("Error: '{}' is not a valid number.", self.input);
        }
        self.input.clear();
    }

    fn subtract(&mut self) {
        if let Ok(value) = self.input.parse::<f64>() {
            self.memory -= value;
            self.message = format!("Result: {:.2}", self.memory);
        } else {
            self.message = format!("Error: '{}' is not a valid number.", self.input);
        }
        self.input.clear();
    }

    fn multiply(&mut self) {
        if let Ok(value) = self.input.parse::<f64>() {
            self.memory *= value;
            self.message = format!("Result: {:.2}", self.memory);
        } else {
            self.message = format!("Error: '{}' is not a valid number.", self.input);
        }
        self.input.clear();
    }

    fn divide(&mut self) {
        if let Ok(value) = self.input.parse::<f64>() {
            if value == 0.0 {
                self.message = "Error: Division by zero is not allowed.".to_string();
            } else {
                self.memory /= value;
                self.message = format!("Result: {:.2}", self.memory);
            }
        } else {
            self.message = format!("Error: '{}' is not a valid number.", self.input);
        }
        self.input.clear();
    }

    fn reset(&mut self) {
        self.memory = 0.0;
        self.message = "Memory reset to 0.0".to_string();
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Memory: {:.2}", self.memory));
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Enter a number:");
                ui.text_edit_singleline(&mut self.input);
            });

            ui.horizontal(|ui| {
                if ui.button("+").clicked() {
                    self.add();
                }
                if ui.button("-").clicked() {
                    self.subtract();
                }
                if ui.button("*").clicked() {
                    self.multiply();
                }
                if ui.button("/").clicked() {
                    self.divide();
                }
                if ui.button("Reset").clicked() {
                    self.reset();
                }
            });

            ui.separator();
            ui.label(&self.message);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Box::new(CalculatorApp::new())),
    )
}
