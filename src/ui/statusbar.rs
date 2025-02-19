use egui::{Align, Button, Layout, Ui};

use crate::data::{commands::CommandItem, config::Config, Group};

pub struct StatusBar {
    pub show_about: bool,
    pub group: (bool, Group),
    pub cmd: (bool, CommandItem),
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            show_about: false,
            group: (false, Group::default()),
            cmd: (false, CommandItem::default()),
        }
    }

    pub fn show(&mut self, ui: &mut Ui, config: &mut Config) -> anyhow::Result<()> {
        self.about_window(ui);
        self.add_group_window(ui, config);
        self.add_command_window(ui, config);
        ui.horizontal(|ui| {
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
                    });
                ui.button("Add Command")
                    .on_hover_text("Add a new command")
                    .clicked()
                    .then(|| {
                        self.cmd.0 = true;
                    });

                ret
            })
            .inner
        })
        .inner
    }
}
