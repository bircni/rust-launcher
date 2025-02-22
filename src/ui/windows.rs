use egui::{Align2, ComboBox, Frame, Image, TextEdit, Ui, Window, include_image, vec2};

use crate::data::{Group, commands::CommandItem, config::Config};

use super::statusbar::StatusBar;

fn generate_window<'a>(ui: &Ui, name: &str, open: &'a mut bool) -> Window<'a> {
    Window::new(name)
        .resizable(false)
        .collapsible(false)
        .open(open)
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .fixed_size(vec2(200.0, 150.0))
        .frame(Frame::window(ui.style()).fill(ui.style().visuals.widgets.open.weak_bg_fill))
}

impl StatusBar {
    pub fn add_command_window(&mut self, ui: &Ui, config: &mut Config) {
        let mut close_window = false;

        generate_window(ui, "Add Command", &mut self.cmd.0).show(ui.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Group:");
                    ComboBox::from_label("")
                        .selected_text(config.groups[self.cmd.1.group_id as usize].name.to_string())
                        .show_ui(ui, |ui| {
                            for group in &config.groups {
                                ui.selectable_value(
                                    &mut self.cmd.1.group_id,
                                    group.id,
                                    &group.name,
                                );
                            }
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Command Name:");
                    ui.add(TextEdit::singleline(&mut self.cmd.1.name).hint_text("Name"));
                });
                ui.horizontal(|ui| {
                    ui.label("Command:");
                    ui.add(TextEdit::multiline(&mut self.cmd.1.command).hint_text("Command"));
                });
                ui.horizontal(|ui| {
                    ui.label("Working dir:");
                    ui.add(
                        TextEdit::singleline(&mut self.cmd.1.working_dir).hint_text("Working dir"),
                    );
                });
                ui.horizontal(|ui| {
                    if ui.button("Add").on_hover_text("Add a command").clicked() {
                        config.add_command(self.cmd.1.clone());
                        close_window = true;
                    }
                });
            });
        });

        if close_window {
            self.cmd.0 = false;
            self.cmd.1 = CommandItem::default();
        }
    }

    pub fn add_group_window(&mut self, ui: &Ui, config: &mut Config) {
        let mut close_window = false;

        generate_window(ui, "Add Group", &mut self.group.0).show(ui.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add(
                    TextEdit::singleline(&mut self.group.1.name)
                        .hint_text("Group Name")
                        .id_salt("group_name"),
                );
                ui.horizontal(|ui| {
                    if ui.button("Add").on_hover_text("Add a group").clicked() {
                        config.add_group(self.group.1.clone());
                        close_window = true;
                    }
                });
            });
        });

        if close_window {
            self.group.0 = false;
            self.group.1 = Group::default();
        }
    }

    pub fn about_window(&mut self, ui: &Ui) {
        generate_window(ui, "About", &mut self.show_about).show(ui.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add(
                    Image::new(include_image!("../../res/icon.png"))
                        .shrink_to_fit()
                        .corner_radius(10.0),
                );

                ui.label(format!("{}: {}", "Version", env!("CARGO_PKG_VERSION")));

                ui.hyperlink_to("Built with egui", "https://docs.rs/egui/");
            });
        });
    }
}
