use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum AvailableCommand {
    #[strum(serialize = "GET")]
    GET,
    #[strum(serialize = "SET")]
    SET,
    #[strum(serialize = "DEL")]
    DEL,
    #[strum(serialize = "PING")]
    PING,
    #[strum(serialize = "STATS")]
    STATS,
}
