// #![windows_subsystem = "windows"]

use std::process::exit;

use funnylauncher::{
    gui::update_screen::UpdateScreen, launcher::launcher_controller::LauncherController,
    utils::log::init_logger,
};
use log::{error, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger().unwrap_or_else(|e| {
        msgbox::create(
            "Fatal error",
            &format!("Couldn't initialize logging system. Error: {}", e),
            msgbox::IconType::Error,
        )
        .unwrap();
        exit(-1);
    });

    info!("Starting launcher...");

    let mut update_screen = UpdateScreen::default();
    update_screen.run().unwrap_or_else(|e| {
        error!("{:?}", e);
        msgbox::create(
            "Fatal error",
            &format!("Couldn't initialize update screen. Error: {:?}", e),
            msgbox::IconType::Error,
        )
        .unwrap_or_else(|e| error!("{e}"));
        exit(-1);
    });

    let mut launcher_controller = LauncherController::new();
    launcher_controller.run().unwrap_or_else(|e| {
        error!("{:?}", e);
        msgbox::create(
            "Fatal error",
            &format!("Couldn't initialize launcher screen. Error: {:?}", e),
            msgbox::IconType::Error,
        )
        .unwrap_or_else(|e| error!("{e}"));
        exit(-1);
    });

    info!("Good bye!");

    Ok(())
}
