use druid::lens::{self, LensExt};
use druid::{
    widget::{Button, Container, Flex, Label, List, Scroll, TextBox, WidgetExt},
    ExtEventSink, FileDialogOptions, Target, Widget,
};
use std::sync::Arc;
use std::thread;

use crate::app::grouper::grouper;

use super::{events::CMD_GROUPER, state::AppState};

fn run_grouper(sink: ExtEventSink, data: String) {
    // Run grouper in a new thread
    thread::spawn(move || {
        let groups = grouper(&data);
        sink.submit_command(CMD_GROUPER, groups, Target::Auto)
            .expect("Failed to submit command")
    })
    .join()
    .unwrap();
}

pub fn build_ui() -> impl Widget<AppState> {
    Container::new(
        Flex::column()
            .with_child(
                TextBox::new()
                    .with_placeholder("Directory")
                    .on_click(|ctx, _, _| {
                        // https://github.com/linebender/druid/blob/master/druid/examples/open_save.rs
                        let open_dialog_options = FileDialogOptions::new()
                            .select_directories()
                            .name_label("Target")
                            .title("Choose a folder")
                            .button_text("Scan");

                        ctx.submit_command(
                            druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()),
                        )
                    })
                    .lens(AppState::selected_path),
            )
            .with_flex_child(
                Button::new("Scan Files")
                    .on_click(|ctx, data: &mut String, _| {
                        run_grouper(ctx.get_external_handle(), data.to_owned());
                    })
                    .lens(AppState::selected_path),
                1.0,
            )
            .with_flex_child(
                Scroll::new(List::new(|| {
                    Label::dynamic(|data: &String, _| data.to_string()).padding(10.0)
                }))
                .lens(lens::Identity.map(
                    |data: &AppState| data.file_groups.keys().clone(),
                    |data: &mut AppState, groups: Arc<HashMap<String, Vec<String>>>| {},
                )),
                1.0,
            ),
    )
}
