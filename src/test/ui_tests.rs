use egui::{accesskit::Role, ThemePreference};
use egui_kittest::{kittest::Queryable, Harness};

use crate::{data::config::Config, ui::App};

pub fn app() -> Harness<'static> {
    let mut app = None;

    Harness::new(move |ctx| {
        let app_instance = app.get_or_insert_with(|| App::new(ctx, Config::default()));
        app_instance.show(ctx);
    })
}

#[tokio::test]
pub async fn test_main_view() {
    let themes = vec![ThemePreference::Dark, ThemePreference::Light];
    for theme in themes {
        let mut harness = app();
        harness.ctx.set_theme(theme);
        harness.run();
        harness.snapshot(&format!("main_view_{theme:?}"));
        harness
            .get_by_role_and_label(Role::Button, "Add Group")
            .click();
        harness.run();

        harness.snapshot(&format!("add_group_{theme:?}"));
        // let label = "Group Name";
        // harness
        //     .get_by_role_and_label(Role::TextInput, "Group Name")
        //     .type_text(label);

        harness
            .get_by_role_and_label(Role::Button, "Close window")
            .click();
        harness.run();

        harness.snapshot(&format!("add_group_close_{theme:?}"));

        harness
            .get_by_role_and_label(Role::Button, "Add Command")
            .click();
        harness.run();

        harness.snapshot(&format!("add_command_{theme:?}"));

        harness
            .get_by_role_and_label(Role::Button, "Close window")
            .click();
        harness.run();

        harness.snapshot(&format!("add_command_close_{theme:?}"));

        harness.get_by_role_and_label(Role::Button, " ? ").click();
        harness.run();

        harness.snapshot(&format!("about_{theme:?}"));
    }
}
