#[derive(PartialEq)]
pub enum Command {
    RUN,      // Launch minecraft
    CONTINUE, // Shutdown game and launch launcher
    VALIDATE,
    DOWNLOAD((u64, u64)),
    UNZIPING,
    PLAY,
    ERROR(String),
    NONE, // Nothing
    EXIT, // Exit from launcher
}
