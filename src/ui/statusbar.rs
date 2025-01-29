use egui::{vec2, Align, Align2, Button, ComboBox, Frame, Layout, TextEdit, Ui, Window};

use crate::data::{CommandItem, Config, Group};

pub struct StatusBar {
    show_about: bool,
    group: (bool, Group),
    cmd: (bool, CommandItem),
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            show_about: false,
            group: (false, Group::default()),
            cmd: (false, CommandItem::default()),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut Config,
        // selected_group: &i32,
    ) -> anyhow::Result<()> {
        self.about_window(ui);
        self.add_group_window(ui, config);
        self.add_command_window(ui, config);
        ui.horizontal(|ui| {
            // egui::ComboBox::from_label("Select one!")
            //     .selected_text(config.groups[*selected_group as usize].name.to_string())
            //     .show_ui(ui, |ui| {
            //         for group in &config.groups {
            //             ui.selectable_value(selected_group, group.id, &group.name);
            //         }
            //     });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add(Button::new(" ? ").corner_radius(40.0))
                    .clicked()
                    .then(|| self.show_about = true);
                let ret = ui
                    .add(Button::new("Save"))
                    .clicked()
                    .then(|| match config.save() {
                        Ok(()) => anyhow::Ok(()),
                        Err(e) => anyhow::bail!(e),
                    })
                    .unwrap_or(Ok(()));

                ui.button("Add Group")
                    .on_hover_text("Add a new group")
                    .clicked()
                    .then(|| {
                        self.group.0 = true;
                        // config.add_group();
                    });
                ui.button("Add Command")
                    .on_hover_text("Add a new command")
                    .clicked()
                    .then(|| {
                        // config.add_command(0);
                        self.cmd.0 = true;
                    });

                ret
            })
            .inner
        })
        .inner
    }

    fn add_group_window(&mut self, ui: &Ui, config: &mut Config) {
        let mut close_window = false;

        Window::new("Add Group")
            .resizable(false)
            .collapsible(false)
            .open(&mut self.group.0)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .fixed_size(vec2(200.0, 150.0))
            .frame(Frame::window(ui.style()).fill(ui.style().visuals.widgets.open.weak_bg_fill))
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Group Name:");
                    ui.text_edit_singleline(&mut self.group.1.name);
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

    fn add_command_window(&mut self, ui: &Ui, config: &mut Config) {
        let mut close_window = false;

        Window::new("Add Command")
            .resizable(false)
            .collapsible(false)
            .open(&mut self.cmd.0)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .fixed_size(vec2(200.0, 150.0))
            .frame(Frame::window(ui.style()).fill(ui.style().visuals.widgets.open.weak_bg_fill))
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Group:");
                        ComboBox::from_label("")
                            .selected_text(
                                config.groups[self.cmd.1.group_id as usize].name.to_string(),
                            )
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
                            TextEdit::singleline(&mut self.cmd.1.working_dir)
                                .hint_text("Working dir"),
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

    fn about_window(&mut self, ui: &Ui) {
        Window::new("About")
            .resizable(false)
            .collapsible(false)
            .open(&mut self.show_about)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .fixed_size(vec2(200.0, 150.0))
            .frame(Frame::window(ui.style()).fill(ui.style().visuals.widgets.open.weak_bg_fill))
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    // ui.add(
                    //     Image::new(include_image!("../../res/icon.png"))
                    //         .shrink_to_fit()
                    //         .rounding(10.0),
                    // );

                    ui.label(format!("{}: {}", "Version", env!("CARGO_PKG_VERSION")));

                    ui.hyperlink_to("Built with egui", "https://docs.rs/egui/");
                });
            });
    }
}
