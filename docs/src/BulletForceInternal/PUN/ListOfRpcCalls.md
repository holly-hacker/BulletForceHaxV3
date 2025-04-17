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

### 0: `AcknowledgeDamageDoneRPC(string status, float damage, int victimID)` {#AcknowledgeDamageDoneRPC}

Class: `PlayerScript`

`status` is most likely a stringified version of `PlayerHitPlayerStatus`.


### 1: `AnotherRPCMethod(?)` {#AnotherRPCMethod}

Not found in source code.


### 2: `BecomeNewMasterClient()` {#BecomeNewMasterClient}

Class: `MasterClientFinder`


### 3: `ChangeCrouchState(?)` {#ChangeCrouchState}

### 4: `Chat(?)` {#Chat}

Class: `InRoomChat`


### 5: `CmdGetTeamNumber(?)` {#CmdGetTeamNumber}

### 6: `ColorRpc(?)` {#ColorRpc}

Class: `OnClickRequestOwnership`


### 7: `DestroyRpc(?)` {#DestroyRpc}

Class: `OnClickDestroy`


### 8: `DisplayVoteData(?)` {#DisplayVoteData}

### 9: `DoJump(?)` {#DoJump}

Class: `JumpAndRunMovement`


### 10: `FetchCheaters(?)` {#FetchCheaters}

### 11: `FetchVoteData(?)` {#FetchVoteData}

### 12: `FlagOwnerTeamUpdated(?)` {#FlagOwnerTeamUpdated}

### 13: `FlagTakenValueUpdated(?)` {#FlagTakenValueUpdated}

### 14: `Flash(?)` {#Flash}

Class: `OnClickFlashRpc`


### 15: `GetBestSpawnPointForPlayer(?)` {#GetBestSpawnPointForPlayer}

### 16: `GotKillAssist(?)` {#GotKillAssist}

### 17: `HealthUpdated(float value)` {#HealthUpdated}

Class: `PlayerScript`


### 18: `InstantiateRpc(?)` {#InstantiateRpc}

Class: `ManualPhotonViewAllocator`


### 19: `JSNow()` {#JSNow}

Class: `PlayerScript`


### 20: `KickPlayer(?)` {#KickPlayer}

### 21: `LatencyReceive(?)` {#LatencyReceive}

Class: `MasterClientFinder`


### 22: `LatencySend(?)` {#LatencySend}

Class: `MasterClientFinder`


### 23: `localCreateGrenade(?)` {#localCreateGrenade}

### 24: `localHurt(?)` {#localHurt}

### 25: `localReload(?)` {#localReload}

### 26: `localSpawnThrowingWeapon(?)` {#localSpawnThrowingWeapon}

### 27: `MapVotedFor(?)` {#MapVotedFor}

### 28: `Marco(?)` {#Marco}

Not found in source code.


### 29: `MatchOverChanged(?)` {#MatchOverChanged}

### 30: `mpMeleeAnimation()` {#mpMeleeAnimation}

Class: `PlayerScript`


### 31: `mpThrowGrenadeAnimation(?)` {#mpThrowGrenadeAnimation}

### 32: `MyRPCMethod(?)` {#MyRPCMethod}

Not found in source code.


### 33: `NukeKill()` {#NukeKill}

Class: `PlayerScript`


### 34: `PickupItemInit(double timeBase, float[] inactivePickupsAndTimes)` {#PickupItemInit}

Class: `PickupItemSyncer`


### 35: `PlayerHitPlayer(?)` {#PlayerHitPlayer}

### 36: `PlayerKickedForPing(short ping)` {#PlayerKickedForPing}

Class: `PlayerScript`


### 37: `Polo(?)` {#Polo}

### 38: `PunPickup(?)` {#PunPickup}

Class: `PickupItem`


### 39: `PunPickupSimple()` {#PunPickupSimple}

Class: `PickupItemSimple`


### 40: `PunRespawn()` / `PunRespawn(Vector3 pos)` {#PunRespawn}

Class: `PickupItem`

It seems like this method has multiple overloads, and can be called either with or without position parameter.


### 41: `ReliabilityMessageReceived(?)` {#ReliabilityMessageReceived}

### 42: `ReliabilityMessageSent(?)` {#ReliabilityMessageSent}

### 43: `RequestForPickupItems()` {#RequestForPickupItems}

Class: `PickupItemSyncer`


### 44: `RequestForPickupTimes()` {#RequestForPickupTimes}

Class: `PickupItemSyncer`


### 45: `RequestVipsOnMasterFromSubordinate(?)` {#RequestVipsOnMasterFromSubordinate}

### 46: `RestartHardcoreModeRound(?)` {#RestartHardcoreModeRound}

### 47: `RestartMatch(?)` {#RestartMatch}

### 48: `RpcDie(int killedPlayerID, int killerID, byte killerHealth, byte killerWeapon, bool headshot)` {#RpcDie}

Class: `PlayerScript`


### 49: `RPCElevatorButtonPressed(?)` {#RPCElevatorButtonPressed}

### 50: `RpcSendChatMessage(string msgUsername, string msg, short r, short g, sthor b)` {#RpcSendChatMessage}

Class: `PlayerScript`

Puts a message in chat in the format similar to `{sender name}: <color=#rgb>{msg}</color>`.

The `msgUsername` parameter seems to be completely ignored. Instead, the game uses the name from the sender's `PlayerProperties`.

If the sender's name is empty, the `: ` section of the chat messages is also removed which allows the sender to place any message in any color in the chat. However, the sender's name can only be empty before the auth token is sent, which means the sender only has a brief window to do this before they are kicked.


### 51: `RpcShoot(int actorID, float damage, Vector3 position, Vector3 direction, byte numberOfBullets, byte spread, double timeShot, int weaponType)` {#RpcShoot}

Class: `PlayerScript`


### 52: `RpcShowHitmarker(?)` {#RpcShowHitmarker}

### 53: `RpcShowPerkMessage(?)` {#RpcShowPerkMessage}

### 54: `SetElevatorsClosed(?)` {#SetElevatorsClosed}

### 55: `SetMaps(?)` {#SetMaps}

### 56: `SetNextMap(?)` {#SetNextMap}

### 57: `SetPing(short p)` {#SetPing}

Class: `PlayerScript`


### 58: `SetRank(byte r)` {#SetRank}

Class: `PlayerScript`


### 59: `SetSpawnPoint(Vector3 s, int spawnedPlayerActorNr)` {#SetSpawnPoint}

Class: `PlayerScript`


### 60: `SetTimeScale(float s)` {#SetTimeScale}

Class: `PlayerScript`


### 61: `ShowAnnouncement(string text, float time)` {#ShowAnnouncement}

Class: `PlayerScript`


### 62: `ShowDebugCapsule(Vector3 pos)` {#ShowDebugCapsule}

Class: `PlayerScript`


### 63: `SpawnFailed(?)` {#SpawnFailed}

### 64: `TaggedPlayer(?)` {#TaggedPlayer}

### 65: `TeleportToPosition(Vector3 position)` {#TeleportToPosition}

Class: `PlayerScript`


### 66: `UpdateAlivePlayers(?)` {#UpdateAlivePlayers}

### 67: `UpdateHMFFARounds(?)` {#UpdateHMFFARounds}

### 68: `UpdateMPDeaths(?)` {#UpdateMPDeaths}

### 69: `UpdateMPKills(?)` {#UpdateMPKills}

### 70: `UpdateMPRounds(?)` {#UpdateMPRounds}

### 71: `UpdateTeamNumber(byte value)` {#UpdateTeamNumber}

Class: `ManualPhotonViewAllocator`


### 72: `UpdateTeamPoints(?)` {#UpdateTeamPoints}

### 73: `UpdateTimeInMatch(?)` {#UpdateTimeInMatch}

### 74: `UpdateVIPsOnSubordinates(?)` {#UpdateVIPsOnSubordinates}

### 75: `UsernameChanged(?)` {#UsernameChanged}

### 76: `WeaponCamoChanged(?)` {#WeaponCamoChanged}

### 77: `WeaponTypeChanged(?)` {#WeaponTypeChanged}

### 78: `RpcACKill(?)` {#RpcACKill}

### 79: `RpcForceKillstreak(?)` {#RpcForceKillstreak}

### 80: `RpcDownloadScriptable(?)` {#RpcDownloadScriptable}

### 81: `DecreaseCountDown()` {#DecreaseCountDown}

Class: `RPCDataSendingMasterClientToMaster`


### 82: `IncreaseNumber()` {#IncreaseNumber}

Class: `RPCDataSendingMasterClientToMaster`


### 83: `SendNewCountDownToClients()` {#SendNewCountDownToClients}

Class: `RPCDataSendingMasterClientToMaster`


### 84: `SetKD(float kd)` {#SetKD}

Class: `PlayerScript`


### 85: `RpcHitVerified(string shotID, bool verified, string info)` {#RpcHitVerified}

Class: `PlayerScript`


### 86: `RpcVerifyHit(string shotID, int damagerID, int damagedPlayerID, int weaponTypeID, float bulletStartPosX, float bulletStartPosY, float bulletStartPosZ, float bulletHitPosX, float bulletHitPosY, float bulletHitPosZ)` {#RpcVerifyHit}

Class: `PlayerScript`


### 87: `RpcRespawned(Vector3 spawnPoint)` {#RpcRespawned}

Class: `PlayerScript`


### 88: `RpcSendMultiplayerAuthToken(string token)` {#RpcSendMultiplayerAuthToken}

This RPC method has to be called using the response string from API endpoint `https://server.blayzegames.com/OnlineAccountSystem/get_multiplayer_auth_code.php`.

<!-- TODO: link to API documentation page -->

If it is not called, the player gets kicked from the game. If this gets called with an invalid auth token (eg. non-existent, from another player or a newer token has already been generated), then the player also gets kicked from the game.


### 89: `RpcRequestEnterVehicle(?)` {#RpcRequestEnterVehicle}

Class: `BFVehicle`


### 90: `RpcEnteredVehicle(?)` {#RpcEnteredVehicle}

Class: `BFVehicle`


### 91: `RpcGetKicked(string reason)` {#RpcGetKicked}

Class: `PlayerScript`


### 92: `RpcReportHack(int reportID, int hackerID, int hackType)` {#RpcReportHack}

Class: `PlayerScript`


### 93: `RpcPlayerHitPlayerHitmarker(int damagerID, int damagedPlayerID, byte weaponType, bool headshot)` {#RpcPlayerHitPlayerHitmarker}

Class: `PlayerScript`


### 94: `AllPlayersInRoomExist(?)` {#AllPlayersInRoomExist}

Not found in source code.


### 95: `CheckPlayers(?)` {#CheckPlayers}

Not found in source code.


### 96: `ReportPlayerHacking(?)` {#ReportPlayerHacking}

Not found in source code.


### 97: `UpdatePlayerVector3(?)` {#UpdatePlayerVector3}

### 98: `SetKeysValues(?)` {#SetKeysValues}

### 99: `RpcDamageVehicle(?)` {#RpcDamageVehicle}

Class: `BFVehicle`


### 100: `RpcVehicleDestroyed(?)` {#RpcVehicleDestroyed}

Class: `BFVehicle`


### 101: `BFS_SetSpawnPointForPlayer(?)` {#BFS_SetSpawnPointForPlayer}

### 102: `BFS_GetBestSpawnPointForPlayer(?)` {#BFS_GetBestSpawnPointForPlayer}

### 103: `BFS_PlayerLeftRoom(?)` {#BFS_PlayerLeftRoom}

### 104: `BFS_FireBullet(?)` {#BFS_FireBullet}

### 105: `BFS_DamagePlayer(?)` {#BFS_DamagePlayer}

### 106: `RpcSendReceiveCustomMapToServer(?)` {#RpcSendReceiveCustomMapToServer}

### 107: `RpcCustomMapLikeDislike(?)` {#RpcCustomMapLikeDislike}
