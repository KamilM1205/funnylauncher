// #![windows_subsystem = "windows"]

use std::process::exit;

use funnylauncher::{
    gui::{login_screen::LoginScreen, update_screen::UpdateScreen},
    launcher::{
        config::AppConfig,
        launcher_controller::{self, LauncherController},
        locale::Locale,
    },
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

    info!("Loading configuration...");

    let config = AppConfig::get_config().unwrap_or_else(|e| {
        error!("{e}");
        msgbox::create("Fatal error", &e, msgbox::IconType::Error).unwrap();
        exit(-1);
    });

    info!("Configuration load complete.");

    info!("Loading locale: \"{}\"...", config.locale);

    let locale = Locale::load(&config.locale);

    info!("Locale load complete.");

    let mut update_screen = UpdateScreen::default();
    update_screen.run(locale.clone()).unwrap_or_else(|e| {
        error!("{:?}", e);
        msgbox::create(
            "Fatal error",
            &format!("Couldn't initialize update screen. Error: {:?}", e),
            msgbox::IconType::Error,
        )
        .unwrap_or_else(|e| error!("{e}"));
        exit(-1);
    });

    info!("Showing login screen.");

    'rerun: loop {
        let mut login_screen = LoginScreen::new(locale.clone());
        login_screen.run().unwrap_or_else(|e| {
            error!("{:?}", e);
            msgbox::create(
                "Fatal error",
                &format!("Couldn't initialize update screen. Error: {:?}", e),
                msgbox::IconType::Error,
            )
            .unwrap_or_else(|e| error!("{e}"));
            exit(-1);
        });

        let launcher_controller = LauncherController::new(locale.clone());
        if let Err(e) = launcher_controller {
            error!("{:?}", e);
            msgbox::create(
                "Error",
                &format!("Your session has timed out. Error: {e}"),
                msgbox::IconType::Error,
            )
            .unwrap_or_else(|e| error!("{e}"));
            continue 'rerun;
        }

        let mut launcher_controller = launcher_controller.unwrap();

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

        break 'rerun;
    }
    info!("Good bye!");

    Ok(())
}
