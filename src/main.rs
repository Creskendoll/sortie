use std::sync::Arc;

use druid::{
    commands::OPEN_FILE, AppDelegate, AppLauncher, Command, DelegateCtx, Env, Handled, Target,
    WindowDesc,
};
use ui::{
    build::build_ui,
    events::CMD_GROUPER,
    state::{initial_data, AppState},
};

mod app;
mod ui;
mod util;

struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(groups) = cmd.get(CMD_GROUPER) {
            data.file_groups = Arc::new(groups.clone());
            for (group, files) in groups {
                println!("{:?}, {} items", group, files.len());
            }
            data.groups = Arc::new(groups.keys().cloned().collect());
            return Handled::Yes;
        }

        if let Some(path) = cmd.get(OPEN_FILE) {
            data.selected_path = path.path().to_str().unwrap().to_string();
            return Handled::Yes;
        }
        Handled::No
    }
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("Sortie");

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .log_to_console()
        .launch(initial_data())
        .expect("Failed to launch application");
}
