use eframe::egui;

pub struct MinimalApp {
    pub name: String,
    pub counter: i32,
    pub dark_mode: bool,
}

impl Default for MinimalApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
            counter: 0,
            dark_mode: true,
        }
    }
}

impl MinimalApp {
    pub fn increment(&mut self) {
        self.counter += 1;
    }

    pub fn decrement(&mut self) {
        self.counter -= 1;
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }
}

impl eframe::App for MinimalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Minimal Cross-Platform GUI");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add_space(8.0);
            ui.label(format!("Hello, {}!", self.name));

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("Increment").clicked() {
                    self.increment();
                }
                if ui.button("Decrement").clicked() {
                    self.decrement();
                }
                if ui.button("Reset").clicked() {
                    self.reset();
                }
                ui.label(format!("Count: {}", self.counter));
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(4.0);

            ui.checkbox(&mut self.dark_mode, "Dark Mode");

            ui.add_space(12.0);
            ui.label(format!("OS: {}", std::env::consts::OS));
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Oberon+ Development System"),
        ..Default::default()
    };

    eframe::run_native(
        "Minimal GUI App",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(MinimalApp::default()))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let app = MinimalApp::default();
        assert_eq!(app.name, "World");
        assert_eq!(app.counter, 0);
        assert!(app.dark_mode);
    }

    #[test]
    fn test_counter_operations() {
        let mut app = MinimalApp::default();
        app.increment();
        assert_eq!(app.counter, 1);
        app.decrement();
        assert_eq!(app.counter, 0);
        app.decrement();
        assert_eq!(app.counter, -1);
        app.reset();
        assert_eq!(app.counter, 0);
    }
}
