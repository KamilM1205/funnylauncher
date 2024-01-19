#[derive(PartialEq)]
pub enum Command {
    RUN,      // Launch minecraft
    CONTINUE, // Shutdown game and launch launcher
    VALIDATE,
    DOWNLOAD((u64, u64)),
    UNZIPING,
    PLAY,
    NONE,     // Nothing
    EXIT,     // Exit from launcher
}
