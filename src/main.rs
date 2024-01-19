// #![windows_subsystem = "windows"]

use funnylauncher::{
    gui::update_screen::UpdateScreen, launcher::launcher_controller::LauncherController,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut update_screen = UpdateScreen::default();
    update_screen.run().unwrap();

    let mut launcher_controller = LauncherController::new();
    launcher_controller.run().unwrap();
    Ok(())
}
