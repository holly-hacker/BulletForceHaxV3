use std::path::PathBuf;

use argh::FromArgs;
use bulletforce_client::Region;

#[derive(FromArgs)]
/// BulletForceHax v3 bot CLI
pub struct RootArgs {
    #[argh(subcommand)]
    pub sub_command: RootSubCommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum RootSubCommand {
    Bot(BotArgs),
    CheckAccounts(CheckAccountsArgs),
}

#[derive(FromArgs, Clone)]
#[argh(subcommand, name = "bot")]
/// Run a bot program
pub struct BotArgs {
    /// the lobby region
    #[argh(
        option,
        short = 'r',
        from_str_fn(parse_region),
        default = "Region::NorthAmerica"
    )]
    pub region: Region,

    /// the lobby to join
    #[argh(positional)]
    pub lobby_name_segment: String,

    /// the name to use
    #[argh(positional)]
    pub player_name: String,

    /// an optional password to authenticate the player
    #[argh(option)]
    pub password: Option<String>,

    /// an optional password hash to authenticate the player
    #[argh(option)]
    pub password_hash: Option<String>,

    /// an optional auth token to authenticate the player
    #[argh(option)]
    pub auth_token: Option<String>,

    /// the amount of threads/clients to start
    #[argh(option, short = 't', default = "1")]
    pub thread_count: usize,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "check-accounts")]
/// Get information about a list of accounts
pub struct CheckAccountsArgs {
    /// path to a text file with accounts stored in `username:password` format
    #[argh(positional)]
    pub account_file: PathBuf,

    /// whether passwords are stored as SHA512 hashes
    #[argh(switch, long = "hashed")]
    pub passwords_are_hashed: bool,
}

fn parse_region(value: &str) -> Result<Region, String> {
    match value.to_ascii_lowercase().as_str() {
        "na" | "us" | "ca" => Ok(Region::NorthAmerica),
        "eu" => Ok(Region::Europe),
        "as" => Ok(Region::Asia),
        "sa" => Ok(Region::SouthAmerica),
        region => Err(format!("Unknown region: {region}")),
    }
}
