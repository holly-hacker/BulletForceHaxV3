use argh::FromArgs;
use bulletforce_client::Region;

#[derive(FromArgs, Debug, Clone)]
/// BulletForceHax v3 bot CLI
pub struct CliArgs {
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

fn parse_region(value: &str) -> Result<Region, String> {
    match value.to_ascii_lowercase().as_str() {
        "na" | "us" | "ca" => Ok(Region::NorthAmerica),
        "eu" => Ok(Region::Europe),
        "as" => Ok(Region::Asia),
        "sa" => Ok(Region::SouthAmerica),
        region => Err(format!("Unknown region: {region}")),
    }
}
