mod cli_args;
mod connect_util;
mod utils;

use bulletforce_client::{
    game::{GameClient, GameClientSettings},
    lobby::{LobbyClientSettings, LobbyState},
    photon_bulletforce::rpc::BfhRpcCall,
    photon_lib::{
        PhotonHashmap, PhotonObject,
        indexmap::indexmap,
        pun::{
            ViewId,
            lifting::{
                ActorInfo, DestroyPlayerEvent, InstantiationEvent, InstantiationEventData,
                PunEvent, RaiseEventParsed,
            },
        },
    },
};
use cli_args::CliArgs;
use connect_util::drive_client_loop;
use tokio::task::JoinSet;
use tracing::{debug, error, info, trace};
use tracing_subscriber::util::SubscriberInitExt;
use utils::generate_uuid_v4;

#[tokio::main]
async fn main() {
    let args: CliArgs = argh::from_env();
    init_logging();

    let mut join_set = JoinSet::new();
    (0..args.thread_count).for_each(|_| {
        let args = args.clone();
        join_set.spawn(run_client(args));
    });

    _ = join_set.join_all();
}

async fn run_client(args: CliArgs) {
    let settings = LobbyClientSettings {
        app_version: "1.104.5_HC_1.105".into(),
        user_id: generate_uuid_v4(),
        region: args.region,
    };

    let mut last_game_stats = (0, 0, 0, 0);
    let game_client_settings = drive_client_loop(settings, |client| {
        match client.get_state() {
            LobbyState::ReadyNoLobby { .. } => {
                info!("Connected to server, joining lobby");
                if let Err(e) = client.join_lobby() {
                    error!("Error while trying to join lobby: {e}");
                }
            }
            LobbyState::Ready {
                games, app_stats, ..
            } => {
                // log lobby state
                let new_game_stats = match app_stats {
                    Some(app_stats) => (
                        games.len(),
                        app_stats.master_peer_count,
                        app_stats.peer_count,
                        app_stats.room_count,
                    ),
                    None => (games.len(), 0, 0, 0),
                };

                if new_game_stats != last_game_stats {
                    info!(
                        "Game count: {}, master peers: {}, peers: {}, rooms: {}",
                        new_game_stats.0, new_game_stats.1, new_game_stats.2, new_game_stats.3,
                    );

                    last_game_stats = new_game_stats;
                }

                // look for target lobby
                let key = games
                    .iter()
                    .find(|(k, v)| {
                        // bullet force
                        if let Some(PhotonObject::String(str)) = v.custom_properties.get("roomName")
                        {
                            if str.contains(&args.lobby_name_segment) {
                                debug!(matched = str, "Matched bf name");
                                return true;
                            }
                        }

                        // some other game, version is 1.0.x
                        if let Some(PhotonObject::String(str)) = v.custom_properties.get("RoomName")
                        {
                            if str.contains(&args.lobby_name_segment) {
                                debug!(matched = str, "Matched other game 1");
                                return true;
                            }
                        }

                        // platform=mobile, version=newfps-333
                        if let Some(PhotonObject::String(str)) =
                            v.custom_properties.get("matchname")
                        {
                            if str.contains(&args.lobby_name_segment) {
                                debug!(matched = str, "Matched other game 2");
                                return true;
                            }
                        }

                        if k.contains(&args.lobby_name_segment) {
                            debug!(matched = k, "Matched game key");
                            return true;
                        }

                        false
                    })
                    .map(|(k, _v)| k);

                if let Some(key) = key {
                    // found game to join
                    if let Err(e) = client.join_game(key.clone()) {
                        error!("Error while trying to join game: {e}");
                    }
                }
            }
            LobbyState::ReadyToJoinGame {
                token,
                room_name,
                address,
            } => {
                return Some(GameClientSettings {
                    token: token.clone(),
                    room_name: room_name.clone(),
                    address: address.clone(),
                });
            }
            _ => (),
        }

        None
    })
    .await;

    let Some(game_client_settings) = game_client_settings else {
        return;
    };

    info!(
        "Found lobby named '{}' in game {}, can join at address {} with token {}",
        args.lobby_name_segment,
        game_client_settings.room_name,
        game_client_settings.address,
        game_client_settings.token,
    );

    drive_client_loop::<GameClient, _, _>(game_client_settings, |client| {
        #[allow(clippy::single_match)]
        match client.get_state() {
            bulletforce_client::game::GameState::Ready { actor_nr, .. } => {
                let actor_nr = *actor_nr;
                let tag = "bfh";
                let player_name = &args.player_name;
                let player_name = format!("[{tag}]{player_name}");

                info!("Initializing player");

                // destroy self, not actually needed
                client
                    .raise_event(RaiseEventParsed {
                        cache: Some(4),
                        data: PunEvent::DestroyPlayer(Box::new(DestroyPlayerEvent {
                            custom_data: Some(PhotonHashmap(
                                indexmap!(0u8.into() => actor_nr.into()),
                            )),
                        })),
                        actor_list: None,
                        group: None,
                        receiver_group: None,
                        event_forward: None,
                    })
                    .unwrap();

                // instantiate own player body
                client
                    .raise_event(RaiseEventParsed {
                        cache: Some(4),
                        data: PunEvent::Instantiation(Box::new(InstantiationEvent {
                            sender_actor: Some(actor_nr),
                            data: Some(InstantiationEventData {
                                prefab_name: "PlayerBody".into(),
                                server_time: 0,
                                instantiation_id: ViewId(actor_nr * 1000 + 1),
                                ..Default::default()
                            }),
                        })),
                        actor_list: None,
                        group: None,
                        receiver_group: None,
                        event_forward: None,
                    })
                    .unwrap();

                // set props
                client
                    .set_player_properties(ActorInfo {
                        player_name: player_name.clone(),
                        custom_properties: indexmap! {
                            "platform".into() => "WebGLPlayer".to_string().into(),
                            "rank".into() => 123u8.into(),
                            "kd".into() => PhotonObject::Float((10.).into()),
                            "up_to_date_version".into() => "1.104.5_HC".to_string().into(),
                        },
                    })
                    .unwrap();

                // auth, is required to not get kicked from the game
                if let Some(auth_token) = &args.auth_token {
                    client
                        .send_rpc_call(
                            BfhRpcCall::RpcSendMultiplayerAuthToken,
                            &[auth_token.to_string().into()],
                        )
                        .unwrap();
                }
            }
            _ => {}
        }

        None::<()>
    })
    .await;
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(if cfg!(debug_assertions) {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish()
        .init();

    info!("Info log enabled!");
    debug!("Debug log enabled!");
    trace!("Trace log enabled!");
}
