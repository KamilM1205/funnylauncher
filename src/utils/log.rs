use std::error::Error;

use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    init_config, Config,
};

use super::constants::LAUNCHER_DIR;

pub fn init_logger() -> Result<(), Box<dyn Error>> {
    let level = log::LevelFilter::Debug;
    let file_path = dirs::data_dir()
        .ok_or("Data dir not found.")?
        .join(LAUNCHER_DIR)
        .join("funnylauncher.log");

    // Building stdout logger
    let stdout = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new("{h([{l}])}(({t})) - {m}{n}")))
        .build();

    // Logging to log file
    let logfile = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new(
            "{d} {h([{l}])}(({t})) - {m}{n}",
        )))
        .build(file_path)?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .build(level),
        )?;

    let _ = init_config(config)?;

    Ok(())
}
