mod config;

use crate::config::{ShutdownCommands};
use eframe::egui::{Button, Context, CentralPanel, ViewportBuilder};
use eframe::{App, Frame};
use std::process::Command;

#[derive(Default)]
struct Window {
    shutdown_commands: ShutdownCommands,
}

impl Window {
    fn exec_command(&self, command: &String) {
        let result = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", command]).spawn()
        } else {
            Command::new("sh").args(["-c", command]).spawn()
        };
        result.expect("Error executing command").wait().unwrap();
    }
}

impl App for Window {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if self.shutdown_commands.config_success {
                ui.heading("Shutdown Options:");

                let shutdown = Button::new("Shutdown");
                let sleep = Button::new("Sleep");
                let lock = Button::new("Lock");
                let logoff = Button::new("Logoff");
                let restart = Button::new("Restart");

                ui.add_space(10.);

                ui.horizontal(|ui| {
                    if ui.add_sized([75., 25.], lock).clicked() {
                        self.exec_command(&self.shutdown_commands.lock)
                    };
                    if ui.add_sized([75., 25.], logoff).clicked() {
                        self.exec_command(&self.shutdown_commands.logoff)
                    };
                    if ui.add_sized([75., 25.], sleep).clicked() {
                        self.exec_command(&self.shutdown_commands.sleep)
                    };
                    if ui.add_sized([75., 25.], restart).clicked() {
                        self.exec_command(&self.shutdown_commands.restart)
                    };
                    if ui.add_sized([75., 25.], shutdown).clicked() {
                        self.exec_command(&self.shutdown_commands.shutdown)
                    }
                });
            } else {
                ui.heading("Invalid Configuration");
                ui.add_space(10.);
                ui.label("Please check your configuration file");
            }
        });
    }
}

fn main() {
    eframe::run_native(
        "Shutdown Dialogue",
        eframe::NativeOptions {
            viewport: ViewportBuilder::default()
                .with_inner_size([425., 100.])
                .with_resizable(false),
            vsync: true,

            ..Default::default()
        },
        Box::new(|_| {
            Ok(Box::<Window>::new(Window {
                shutdown_commands: ShutdownCommands::new(),
            }))
        }),
    )
    .expect("Window failed to start");
}
