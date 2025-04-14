# List of RPC calls

The RPC indices can be found through [UABEA](https://github.com/nesrak1/UABEA) (or perhaps [UABEANext](https://github.com/nesrak1/UABEANext)?) and checking the `PhotonServerSettings` MonoBehavior resource.

Signatures for [RPC calls](https://doc.photonengine.com/pun/current/gameplay/rpcsandraiseevent) can be found in a generated DummyDLL by looking for methods with the `PunRPC` attribute applied. For example:
```cs
// PlayerScript
[PunRPC]
public void AcknowledgeDamageDoneRPC(string status, float damage, int victimID)
{
	// ...
}
```

The parameters to this function are the parameters sent in the RPC event.

In some cases, these methods may take an additional parameter `PhotonMessageInfo` parameter. This parameter is not sent as part of the RPC event:
```cs
// PlayerScript
[PunRPC]
private void HealthUpdated(float value, PhotonMessageInfo info)
{
	// ...
}
```

## Full list

### 0: `AcknowledgeDamageDoneRPC(string status, float damage, int victimID)`

Class: `PlayerScript`

`status` is most likely a stringified version of `PlayerHitPlayerStatus`.


### 1: `AnotherRPCMethod(?)`

Not found in source code.


### 2: `BecomeNewMasterClient()`

Class: `MasterClientFinder`


### 3: `ChangeCrouchState(?)`

### 4: `Chat(?)`

Class: `InRoomChat`


### 5: `CmdGetTeamNumber(?)`

### 6: `ColorRpc(?)`

Class: `OnClickRequestOwnership`


### 7: `DestroyRpc(?)`

Class: `OnClickDestroy`


### 8: `DisplayVoteData(?)`

### 9: `DoJump(?)`

Class: `JumpAndRunMovement`


### 10: `FetchCheaters(?)`

### 11: `FetchVoteData(?)`

### 12: `FlagOwnerTeamUpdated(?)`

### 13: `FlagTakenValueUpdated(?)`

### 14: `Flash(?)`

Class: `OnClickFlashRpc`


### 15: `GetBestSpawnPointForPlayer(?)`

### 16: `GotKillAssist(?)`

### 17: `HealthUpdated(float value)`

Class: `PlayerScript`


### 18: `InstantiateRpc(?)`

Class: `ManualPhotonViewAllocator`


### 19: `JSNow()`

Class: `PlayerScript`


### 20: `KickPlayer(?)`

### 21: `LatencyReceive(?)`

Class: `MasterClientFinder`


### 22: `LatencySend(?)`

Class: `MasterClientFinder`


### 23: `localCreateGrenade(?)`

### 24: `localHurt(?)`

### 25: `localReload(?)`

### 26: `localSpawnThrowingWeapon(?)`

### 27: `MapVotedFor(?)`

### 28: `Marco(?)`

Not found in source code.


### 29: `MatchOverChanged(?)`

### 30: `mpMeleeAnimation()`

Class: `PlayerScript`


### 31: `mpThrowGrenadeAnimation(?)`

### 32: `MyRPCMethod(?)`

Not found in source code.


### 33: `NukeKill()`

Class: `PlayerScript`


### 34: `PickupItemInit(double timeBase, float[] inactivePickupsAndTimes)`

Class: `PickupItemSyncer`


### 35: `PlayerHitPlayer(?)`

### 36: `PlayerKickedForPing(short ping)`

Class: `PlayerScript`


### 37: `Polo(?)`

### 38: `PunPickup(?)`

Class: `PickupItem`


### 39: `PunPickupSimple()`

Class: `PickupItemSimple`


### 40: `PunRespawn()` / `PunRespawn(Vector3 pos)`

Class: `PickupItem`

It seems like this method has multiple overloads, and can be called either with or without position parameter.


### 41: `ReliabilityMessageReceived(?)`

### 42: `ReliabilityMessageSent(?)`

### 43: `RequestForPickupItems()`

Class: `PickupItemSyncer`


### 44: `RequestForPickupTimes()`

Class: `PickupItemSyncer`


### 45: `RequestVipsOnMasterFromSubordinate(?)`

### 46: `RestartHardcoreModeRound(?)`

### 47: `RestartMatch(?)`

### 48: `RpcDie(int killedPlayerID, int killerID, byte killerHealth, byte killerWeapon, bool headshot)`

Class: `PlayerScript`


### 49: `RPCElevatorButtonPressed(?)`

### 50: `RpcSendChatMessage(string msgUsername, string msg, short r, short g, sthor b)`

Class: `PlayerScript`

Puts a message in chat in the format similar to `{sender name}: <color=#rgb>{msg}</color>`.

The `msgUsername` parameter seems to be completely ignored. Instead, the game uses the name from the sender's `PlayerProperties`.

If the sender's name is empty, the `: ` section of the chat messages is also removed which allows the sender to place any message in any color in the chat. However, the sender's name can only be empty before the auth token is sent, which means the sender only has a brief window to do this before they are kicked.


### 51: `RpcShoot(int actorID, float damage, Vector3 position, Vector3 direction, byte numberOfBullets, byte spread, double timeShot, int weaponType)`

Class: `PlayerScript`


### 52: `RpcShowHitmarker(?)`

### 53: `RpcShowPerkMessage(?)`

### 54: `SetElevatorsClosed(?)`

### 55: `SetMaps(?)`

### 56: `SetNextMap(?)`

### 57: `SetPing(short p)`

Class: `PlayerScript`


### 58: `SetRank(byte r)`

Class: `PlayerScript`


### 59: `SetSpawnPoint(Vector3 s, int spawnedPlayerActorNr)`

Class: `PlayerScript`


### 60: `SetTimeScale(float s)`

Class: `PlayerScript`


### 61: `ShowAnnouncement(string text, float time)`

Class: `PlayerScript`


### 62: `ShowDebugCapsule(Vector3 pos)`

Class: `PlayerScript`


### 63: `SpawnFailed(?)`

### 64: `TaggedPlayer(?)`

### 65: `TeleportToPosition(Vector3 position)`

Class: `PlayerScript`


### 66: `UpdateAlivePlayers(?)`

### 67: `UpdateHMFFARounds(?)`

### 68: `UpdateMPDeaths(?)`

### 69: `UpdateMPKills(?)`

### 70: `UpdateMPRounds(?)`

### 71: `UpdateTeamNumber(byte value)`

Class: `ManualPhotonViewAllocator`


### 72: `UpdateTeamPoints(?)`

### 73: `UpdateTimeInMatch(?)`

### 74: `UpdateVIPsOnSubordinates(?)`

### 75: `UsernameChanged(?)`

### 76: `WeaponCamoChanged(?)`

### 77: `WeaponTypeChanged(?)`

### 78: `RpcACKill(?)`

### 79: `RpcForceKillstreak(?)`

### 80: `RpcDownloadScriptable(?)`

### 81: `DecreaseCountDown()`

Class: `RPCDataSendingMasterClientToMaster`


### 82: `IncreaseNumber()`

Class: `RPCDataSendingMasterClientToMaster`


### 83: `SendNewCountDownToClients()`

Class: `RPCDataSendingMasterClientToMaster`


### 84: `SetKD(float kd)`

Class: `PlayerScript`


### 85: `RpcHitVerified(string shotID, bool verified, string info)`

Class: `PlayerScript`


### 86: `RpcVerifyHit(string shotID, int damagerID, int damagedPlayerID, int weaponTypeID, float bulletStartPosX, float bulletStartPosY, float bulletStartPosZ, float bulletHitPosX, float bulletHitPosY, float bulletHitPosZ)`

Class: `PlayerScript`


### 87: `RpcRespawned(Vector3 spawnPoint)`

Class: `PlayerScript`


### 88: `RpcSendMultiplayerAuthToken(string token)`

This RPC method has to be called using the response string from API endpoint `https://server.blayzegames.com/OnlineAccountSystem/get_multiplayer_auth_code.php`.

<!-- TODO: link to API documentation page -->

If it is not called, the player gets kicked from the game. If this gets called with an invalid auth token (eg. non-existent, from another player or a newer token has already been generated), then the player also gets kicked from the game.


### 89: `RpcRequestEnterVehicle(?)`

Class: `BFVehicle`


### 90: `RpcEnteredVehicle(?)`

Class: `BFVehicle`


### 91: `RpcGetKicked(string reason)`

Class: `PlayerScript`


### 92: `RpcReportHack(int reportID, int hackerID, int hackType)`

Class: `PlayerScript`


### 93: `RpcPlayerHitPlayerHitmarker(int damagerID, int damagedPlayerID, byte weaponType, bool headshot)`

Class: `PlayerScript`


### 94: `AllPlayersInRoomExist(?)`

Not found in source code.


### 95: `CheckPlayers(?)`

Not found in source code.


### 96: `ReportPlayerHacking(?)`

Not found in source code.


### 97: `UpdatePlayerVector3(?)`

### 98: `SetKeysValues(?)`

### 99: `RpcDamageVehicle(?)`

Class: `BFVehicle`


### 100: `RpcVehicleDestroyed(?)`

Class: `BFVehicle`


### 101: `BFS_SetSpawnPointForPlayer(?)`

### 102: `BFS_GetBestSpawnPointForPlayer(?)`

### 103: `BFS_PlayerLeftRoom(?)`

### 104: `BFS_FireBullet(?)`

### 105: `BFS_DamagePlayer(?)`

### 106: `RpcSendReceiveCustomMapToServer(?)`

### 107: `RpcCustomMapLikeDislike(?)`
