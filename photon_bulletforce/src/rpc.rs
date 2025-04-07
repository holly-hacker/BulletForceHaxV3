use strum::EnumProperty as _;

#[repr(u8)]
#[derive(Debug, strum::EnumProperty, strum::IntoStaticStr, strum::FromRepr)]
pub enum BfhRpcCall {
    #[strum(props(Name = "AcknowledgeDamageDoneRPC"))]
    AcknowledgeDamageDoneRPC = 0,
    #[strum(props(Name = "AnotherRPCMethod"))]
    AnotherRPCMethod,
    #[strum(props(Name = "BecomeNewMasterClient"))]
    BecomeNewMasterClient,
    #[strum(props(Name = "ChangeCrouchState"))]
    ChangeCrouchState,
    #[strum(props(Name = "Chat"))]
    Chat,
    #[strum(props(Name = "CmdGetTeamNumber"))]
    CmdGetTeamNumber,
    #[strum(props(Name = "ColorRpc"))]
    ColorRpc,
    #[strum(props(Name = "DestroyRpc"))]
    DestroyRpc,
    #[strum(props(Name = "DisplayVoteData"))]
    DisplayVoteData,
    #[strum(props(Name = "DoJump"))]
    DoJump,
    #[strum(props(Name = "FetchCheaters"))]
    FetchCheaters,
    #[strum(props(Name = "FetchVoteData"))]
    FetchVoteData,
    #[strum(props(Name = "FlagOwnerTeamUpdated"))]
    FlagOwnerTeamUpdated,
    #[strum(props(Name = "FlagTakenValueUpdated"))]
    FlagTakenValueUpdated,
    #[strum(props(Name = "Flash"))]
    Flash,
    #[strum(props(Name = "GetBestSpawnPointForPlayer"))]
    GetBestSpawnPointForPlayer,
    #[strum(props(Name = "GotKillAssist"))]
    GotKillAssist,
    #[strum(props(Name = "HealthUpdated"))]
    HealthUpdated,
    #[strum(props(Name = "InstantiateRpc"))]
    InstantiateRpc,
    #[strum(props(Name = "JSNow"))]
    JSNow,
    #[strum(props(Name = "KickPlayer"))]
    KickPlayer,
    #[strum(props(Name = "LatencyReceive"))]
    LatencyReceive,
    #[strum(props(Name = "LatencySend"))]
    LatencySend,
    #[strum(props(Name = "localCreateGrenade"))]
    LocalCreateGrenade,
    #[strum(props(Name = "localHurt"))]
    LocalHurt,
    #[strum(props(Name = "localReload"))]
    LocalReload,
    #[strum(props(Name = "localSpawnThrowingWeapon"))]
    LocalSpawnThrowingWeapon,
    #[strum(props(Name = "MapVotedFor"))]
    MapVotedFor,
    #[strum(props(Name = "Marco"))]
    Marco,
    #[strum(props(Name = "MatchOverChanged"))]
    MatchOverChanged,
    #[strum(props(Name = "mpMeleeAnimation"))]
    MpMeleeAnimation,
    #[strum(props(Name = "mpThrowGrenadeAnimation"))]
    MpThrowGrenadeAnimation,
    #[strum(props(Name = "MyRPCMethod"))]
    MyRPCMethod,
    #[strum(props(Name = "NukeKill"))]
    NukeKill,
    #[strum(props(Name = "PickupItemInit"))]
    PickupItemInit,
    #[strum(props(Name = "PlayerHitPlayer"))]
    PlayerHitPlayer,
    #[strum(props(Name = "PlayerKickedForPing"))]
    PlayerKickedForPing,
    #[strum(props(Name = "Polo"))]
    Polo,
    #[strum(props(Name = "PunPickup"))]
    PunPickup,
    #[strum(props(Name = "PunPickupSimple"))]
    PunPickupSimple,
    #[strum(props(Name = "PunRespawn"))]
    PunRespawn,
    #[strum(props(Name = "ReliabilityMessageReceived"))]
    ReliabilityMessageReceived,
    #[strum(props(Name = "ReliabilityMessageSent"))]
    ReliabilityMessageSent,
    #[strum(props(Name = "RequestForPickupItems"))]
    RequestForPickupItems,
    #[strum(props(Name = "RequestForPickupTimes"))]
    RequestForPickupTimes,
    #[strum(props(Name = "RequestVipsOnMasterFromSubordinate"))]
    RequestVipsOnMasterFromSubordinate,
    #[strum(props(Name = "RestartHardcoreModeRound"))]
    RestartHardcoreModeRound,
    #[strum(props(Name = "RestartMatch"))]
    RestartMatch,
    #[strum(props(Name = "RpcDie"))]
    RpcDie,
    #[strum(props(Name = "RPCElevatorButtonPressed"))]
    RPCElevatorButtonPressed,
    #[strum(props(Name = "RpcSendChatMessage"))]
    RpcSendChatMessage,
    #[strum(props(Name = "RpcShoot"))]
    RpcShoot,
    #[strum(props(Name = "RpcShowHitmarker"))]
    RpcShowHitmarker,
    #[strum(props(Name = "RpcShowPerkMessage"))]
    RpcShowPerkMessage,
    #[strum(props(Name = "SetElevatorsClosed"))]
    SetElevatorsClosed,
    #[strum(props(Name = "SetMaps"))]
    SetMaps,
    #[strum(props(Name = "SetNextMap"))]
    SetNextMap,
    #[strum(props(Name = "SetPing"))]
    SetPing,
    #[strum(props(Name = "SetRank"))]
    SetRank,
    #[strum(props(Name = "SetSpawnPoint"))]
    SetSpawnPoint,
    #[strum(props(Name = "SetTimeScale"))]
    SetTimeScale,
    #[strum(props(Name = "ShowAnnouncement"))]
    ShowAnnouncement,
    #[strum(props(Name = "ShowDebugCapsule"))]
    ShowDebugCapsule,
    #[strum(props(Name = "SpawnFailed"))]
    SpawnFailed,
    #[strum(props(Name = "TaggedPlayer"))]
    TaggedPlayer,
    #[strum(props(Name = "TeleportToPosition"))]
    TeleportToPosition,
    #[strum(props(Name = "UpdateAlivePlayers"))]
    UpdateAlivePlayers,
    #[strum(props(Name = "UpdateHMFFARounds"))]
    UpdateHMFFARounds,
    #[strum(props(Name = "UpdateMPDeaths"))]
    UpdateMPDeaths,
    #[strum(props(Name = "UpdateMPKills"))]
    UpdateMPKills,
    #[strum(props(Name = "UpdateMPRounds"))]
    UpdateMPRounds,
    #[strum(props(Name = "UpdateTeamNumber"))]
    UpdateTeamNumber,
    #[strum(props(Name = "UpdateTeamPoints"))]
    UpdateTeamPoints,
    #[strum(props(Name = "UpdateTimeInMatch"))]
    UpdateTimeInMatch,
    #[strum(props(Name = "UpdateVIPsOnSubordinates"))]
    UpdateVIPsOnSubordinates,
    #[strum(props(Name = "UsernameChanged"))]
    UsernameChanged,
    #[strum(props(Name = "WeaponCamoChanged"))]
    WeaponCamoChanged,
    #[strum(props(Name = "WeaponTypeChanged"))]
    WeaponTypeChanged,
    #[strum(props(Name = "RpcACKill"))]
    RpcACKill,
    #[strum(props(Name = "RpcForceKillstreak"))]
    RpcForceKillstreak,
    #[strum(props(Name = "RpcDownloadScriptable"))]
    RpcDownloadScriptable,
    #[strum(props(Name = "DecreaseCountDown"))]
    DecreaseCountDown,
    #[strum(props(Name = "IncreaseNumber"))]
    IncreaseNumber,
    #[strum(props(Name = "SendNewCountDownToClients"))]
    SendNewCountDownToClients,
    #[strum(props(Name = "SetKD"))]
    SetKD,
    #[strum(props(Name = "RpcHitVerified"))]
    RpcHitVerified,
    #[strum(props(Name = "RpcVerifyHit"))]
    RpcVerifyHit,
    #[strum(props(Name = "RpcRespawned"))]
    RpcRespawned,
    #[strum(props(Name = "RpcSendMultiplayerAuthToken"))]
    RpcSendMultiplayerAuthToken,
    #[strum(props(Name = "RpcRequestEnterVehicle"))]
    RpcRequestEnterVehicle,
    #[strum(props(Name = "RpcEnteredVehicle"))]
    RpcEnteredVehicle,
    #[strum(props(Name = "RpcGetKicked"))]
    RpcGetKicked,
    #[strum(props(Name = "RpcReportHack"))]
    RpcReportHack,
    #[strum(props(Name = "RpcPlayerHitPlayerHitmarker"))]
    RpcPlayerHitPlayerHitmarker,
    #[strum(props(Name = "AllPlayersInRoomExist"))]
    AllPlayersInRoomExist,
    #[strum(props(Name = "CheckPlayers"))]
    CheckPlayers,
    #[strum(props(Name = "ReportPlayerHacking"))]
    ReportPlayerHacking,
    #[strum(props(Name = "UpdatePlayerVector3"))]
    UpdatePlayerVector3,
    #[strum(props(Name = "SetKeysValues"))]
    SetKeysValues,
    #[strum(props(Name = "RpcDamageVehicle"))]
    RpcDamageVehicle,
    #[strum(props(Name = "RpcVehicleDestroyed"))]
    RpcVehicleDestroyed,
    #[strum(props(Name = "BFS_SetSpawnPointForPlayer"))]
    BfsSetSpawnPointForPlayer,
    #[strum(props(Name = "BFS_GetBestSpawnPointForPlayer"))]
    BfsGetBestSpawnPointForPlayer,
    #[strum(props(Name = "BFS_PlayerLeftRoom"))]
    BfsPlayerLeftRoom,
    #[strum(props(Name = "BFS_FireBullet"))]
    BfsFireBullet,
    #[strum(props(Name = "BFS_DamagePlayer"))]
    BfsDamagePlayer,
    #[strum(props(Name = "RpcSendReceiveCustomMapToServer"))]
    RpcSendReceiveCustomMapToServer,
    #[strum(props(Name = "RpcCustomMapLikeDislike"))]
    RpcCustomMapLikeDislike,
}

impl BfhRpcCall {
    pub fn get_name(&self) -> &'static str {
        self.get_str("Name").unwrap_or_else(|| self.into())
    }
}
