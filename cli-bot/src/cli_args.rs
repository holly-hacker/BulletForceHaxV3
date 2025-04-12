use argh::FromArgs;
use bulletforce_client::Region;

#[derive(FromArgs, Debug)]
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
