use egui::{accesskit::Role, ThemePreference};
use egui_kittest::{kittest::Queryable, Harness};
use wgpu::InstanceDescriptor;

use crate::{data::config::Config, ui::App};

pub fn app() -> Harness<'static> {
    let mut app = None;

    Harness::new(move |ctx| {
        let app_instance = app.get_or_insert_with(|| App::new(ctx, Config::default()));
        app_instance.show(ctx);
    })
}

async fn gpu_available() -> bool {
    wgpu::Instance::new(&InstanceDescriptor::default())
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .is_some()
}

fn command(
    harness: &mut Harness<'static>,
    theme: ThemePreference,
    cmd: &(String, String),
    num: usize,
) {
    harness
        .get_by_role_and_label(Role::Button, "Add Command")
        .click();

    harness.run();

    harness
        .get_all_by_role(Role::TextInput)
        .find(|n| n.placeholder().is_some_and(|s| s == "Name"))
        .unwrap()
        .type_text(&cmd.0);
    harness.run();
    // Workaround for the text not being typed in twice
    harness.get_by_role_and_label(Role::Button, "Add").focus();
    harness.run();
    harness
        .get_all_by_role(Role::MultilineTextInput)
        .find(|n| n.placeholder().is_some_and(|s| s == "Command"))
        .unwrap()
        .type_text(&cmd.1);

    harness.run();
    harness.snapshot(&format!("{}_{theme:?}_input_command", cmd.0));

    harness.get_by_role_and_label(Role::Button, "Add").click();

    harness.run();
    harness.snapshot(&format!("{}_{theme:?}_add_command_end", cmd.0));

    harness
        .get_all_by_role(Role::Button)
        .filter(|n| n.label().is_some_and(|s| s == "Run"))
        .nth(num)
        // .find(|n| n.label().is_some_and(|s| s == "Run"))
        .unwrap()
        .click();

    harness.run();
    harness.snapshot(&format!("{}_{theme:?}_run_command", cmd.0));
}

#[tokio::test]
pub async fn test_main_view() {
    if !gpu_available().await {
        return;
    }

    let themes = vec![ThemePreference::Dark, ThemePreference::Light];

    for theme in themes {
        let commands = [
            ("CMD-1".to_owned(), format!("echo 'Hello {theme:?} Mode'")),
            ("CMD-2".to_owned(), format!("qwertz 'Hello {theme:?} Mode'")), // Should fail
            ("CMD-3".to_owned(), "echo 'Hello World 2'".to_owned()),
        ];

        let mut harness = app();
        harness.ctx.set_theme(theme);

        harness.run();
        harness.snapshot(&format!("{theme:?}_main_view"));

        harness
            .get_by_role_and_label(Role::Button, "Add Group")
            .click();

        harness.run();
        harness.snapshot(&format!("{theme:?}_add_group"));

        harness
            .get_all_by_role(Role::TextInput)
            .find(|a| a.placeholder().is_some_and(|s| s == "Group Name"))
            .unwrap()
            .type_text("Test Group");

        harness.run();
        harness.snapshot(&format!("{theme:?}_input_group_name"));

        harness.get_by_role_and_label(Role::Button, "Add").click();

        harness.run();
        harness.snapshot(&format!("{theme:?}_add_group_end"));

        for (index, cmd) in commands.iter().enumerate() {
            command(&mut harness, theme, cmd, index);
        }

        harness.run();
        harness.snapshot(&format!("{theme:?}_commands_finalized"));

        harness.get_by_role_and_label(Role::Button, " ? ").click();

        harness.run();
        harness.snapshot(&format!("{theme:?}_about_dialog"));
    }
}
