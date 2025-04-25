use std::{
    fs::File,
    io::{BufRead as _, BufReader},
};

use bulletforce_api::utils::response_to_string;
use sha2::{Digest as _, Sha512};
use tracing::{debug, error, info};

use crate::cli_args::CheckAccountsArgs;

pub async fn check_accounts(args: CheckAccountsArgs) {
    let accounts = {
        let accounts = File::open(args.account_file).expect("open accounts file");
        let reader = BufReader::new(accounts);
        reader
            .lines()
            .flat_map(|l| {
                let line = l.expect("line should be valid");

                if line.is_empty() {
                    return None;
                }

                let (username, password) = line.split_once(':').expect("line should contain `:`");

                if args.passwords_are_hashed {
                    Some((username.to_string(), password.to_string()))
                } else {
                    let hashed_password = Sha512::digest(password.as_bytes());
                    let hashed_password = data_encoding::HEXUPPER.encode(&hashed_password);
                    Some((username.to_string(), hashed_password))
                }
            })
            .collect::<Vec<_>>()
    };

    if accounts.is_empty() {
        info!("No accounts provided");
    }

    let api_client = bulletforce_api::Client::default();
    for (username, password_hash) in accounts {
        debug!(username, "Checking account");
        let res = api_client
            .login(&bulletforce_api::types::LoginBody {
                locale: Some("english".to_string()),
                username: Some(bulletforce_api::types::UserName(username.clone())),
                password: Some(bulletforce_api::types::UserPassword(password_hash)),
                store: Some("BALYZE_WEB".into()),
                tutorialr: Some(1.),
                use_json: Some(true),
                crazy_games_token: None,
            })
            .await;

        let res = match res {
            Ok(ok) => response_to_string(ok)
                .await
                .expect("read response to string"),
            Err(e) => {
                error!("Error in response: {e:?}");
                continue;
            }
        };

        info!(username, response = res, "Account checked");
    }

    // todo
}
