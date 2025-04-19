pub mod utils;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

impl Default for Client {
    fn default() -> Self {
        Self::new("https://server.blayzegames.com/OnlineAccountSystem")
    }
}
