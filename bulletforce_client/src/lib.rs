mod clients;
pub mod errors;
mod utils;

pub use clients::*;
pub use photon_lib;

#[derive(Debug, Clone, Copy)]
pub enum Region {
    NorthAmerica,
    Europe,
    Asia,
    SouthAmerica,
}

impl Region {
    pub const fn get_lobby_url(self) -> &'static str {
        match self {
            Region::NorthAmerica => {
                "wss://game-ca-1.blayzegames.com:2053/?libversion=4.1.6.10&sid=30&app="
            }
            Region::Europe => {
                "wss://game-eu-1.blayzegames.com:2053/?libversion=4.1.6.10&sid=30&app="
            }
            Region::Asia => "wss://game-as-1.blayzegames.com:2053/?libversion=4.1.6.10&sid=30&app=",
            Region::SouthAmerica => {
                "wss://game-sa-1.blayzegames.com:2053/?libversion=4.1.6.10&sid=30&app="
            }
        }
    }
}
