#![windows_subsystem = "windows"]

use eframe::egui;

struct SpawnCellApp {
    input_x: String,
    input_y: String,
    result: Option<SpawnCellResult>,
}

struct SpawnCellResult {
    lower_left: (i32, i32),
    lower_right: (i32, i32),
    upper_left: (i32, i32),
    upper_right: (i32, i32),
}

impl Default for SpawnCellApp {
    fn default() -> Self {
        Self {
            input_x: String::new(),
            input_y: String::new(),
            result: None,
        }
    }
}

impl SpawnCellApp {
    fn floor_div(n: i32, d: i32) -> i32 {
        let mut q = n / d;
        let r = n % d;
        if (r != 0) && ((r < 0) != (d < 0)) {
            q -= 1;
        }
        q
    }

    fn calculate_spawn_cell(x: i32, y: i32) -> SpawnCellResult {
        let cell_x = Self::floor_div(x, 16);
        let cell_y = Self::floor_div(y, 16);

        let lower_left = (cell_x * 16, cell_y * 16);
        let lower_right = (cell_x * 16 + 15, cell_y * 16);
        let upper_left = (cell_x * 16, cell_y * 16 + 15);
        let upper_right = (cell_x * 16 + 15, cell_y * 16 + 15);

        SpawnCellResult {
            lower_left,
            lower_right,
            upper_left,
            upper_right,
        }
    }

    fn try_calculate(&mut self, ui: &mut egui::Ui) {
        let parsed_x = self.input_x.trim().parse::<i32>();
        let parsed_y = self.input_y.trim().parse::<i32>();

        match (parsed_x, parsed_y) {
            (Ok(x), Ok(y)) => {
                self.result = Some(Self::calculate_spawn_cell(x, y));
            }
            _ => {
                self.result = None;
                ui.label("Please enter valid integer coordinates!");
            }
        }
    }
}

impl eframe::App for SpawnCellApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Exit app immediately on Escape key press
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            std::process::exit(0);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Core Keeper Spawn Cell Calculator");

            let mut enter_pressed = false;

            ui.horizontal(|ui| {
                ui.label("Enter X coordinate:");
                let response_x = ui.text_edit_singleline(&mut self.input_x);
                if response_x.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    enter_pressed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Enter Y coordinate:");
                let response_y = ui.text_edit_singleline(&mut self.input_y);
                if response_y.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    enter_pressed = true;
                }
            });

            if ui.button("Calculate").clicked() || enter_pressed {
                self.try_calculate(ui);
            }

            ui.separator();

            if let Some(res) = &self.result {
                ui.label(format!("Lower-left corner:  {:?}", res.lower_left));
                ui.label(format!("Lower-right corner: {:?}", res.lower_right));
                ui.label(format!("Upper-left corner:  {:?}", res.upper_left));
                ui.label(format!("Upper-right corner: {:?}", res.upper_right));

                ui.label("");
                ui.label("Visual representation:");

                ui.label(format!(
                    "( {:?} )-----------( {:?} )",
                    res.upper_left, res.upper_right
                ));
                ui.label("   |                   |");
                ui.label("   |    Spawn Cell     |");
                ui.label("   |                   |");
                ui.label(format!(
                    "( {:?} )-----------( {:?} )",
                    res.lower_left, res.lower_right
                ));
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Core Keeper Spawn Cell Calculator",
        options,
        Box::new(|_cc| Box::new(SpawnCellApp::default())),
    )
}
