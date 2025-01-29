use eframe::CreationContext;
use egui::{vec2, CentralPanel, Color32, Context, ScrollArea, SidePanel, TextEdit, TextStyle};
use statusbar::StatusBar;
use std::f32;

use crate::data::{CommandResult, Config};

mod statusbar;

pub struct App {
    config: Config,
    statusbar: StatusBar,
    // selected_group: i32,
    cmd_result: CommandResult,
}

impl App {
    pub fn new(cc: &CreationContext<'_>, config: Config) -> Self {
        cc.egui_ctx.style_mut(|s| {
            s.text_styles.insert(
                TextStyle::Name("subheading".into()),
                TextStyle::Monospace.resolve(s),
            );
            s.text_styles
                .insert(TextStyle::Body, TextStyle::Monospace.resolve(s));
            s.spacing.item_spacing = vec2(10.0, f32::consts::PI * 1.76643);
        });

        Self {
            config,
            statusbar: StatusBar::new(),
            // selected_group: 0,
            cmd_result: CommandResult::default(),
        }
    }
}

/// Main application loop (called every frame)
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        SidePanel::right("Side Panel").show(ctx, |ui| {
            ui.heading("Last Command Result");
            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                ui.label("Command:");
                ui.monospace(&self.cmd_result.executed_cmd);
                ui.label(format!("Exit code: {}", self.cmd_result.status));
                ui.label("Stdout:");
                ui.add(TextEdit::multiline(&mut self.cmd_result.stdout));
                ui.label("Stderr:");
                ui.add(TextEdit::multiline(&mut self.cmd_result.stderr).text_color(Color32::RED));
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            self.statusbar
                .show(ui, &mut self.config)
                .unwrap_or_else(|e| {
                    println!("{e:?}");
                    // se'lf.toasts.error(e.to_string());
                });
            ui.vertical_centered(|ui| {
                ui.separator();
            });
            ScrollArea::both().show(ui, |ui| {
                for group in &self.config.groups {
                    ui.heading(&group.name);
                    for command in &self.config.commands {
                        if command.group_id == group.id {
                            ui.horizontal(|ui| {
                                ui.button("Run").clicked().then(|| {
                                    println!("Running command: {}", command.command);
                                    self.cmd_result = command.run();
                                });
                                ui.label(format!("{} | {}", command.name, command.command));
                            });
                        }
                    }
                }
            });
        });
    }
}
