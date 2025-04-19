# List of RPC calls

The RPC indices can be found through [UABEA](https://github.com/nesrak1/UABEA) and checking the `PhotonServerSettings` MonoBehavior resource.

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

In some cases, these methods may take an additional parameter `PhotonMessageInfo` parameter. This parameter isn't sent as part of the RPC event:
```cs
// PlayerScript
[PunRPC]
private void HealthUpdated(float value, PhotonMessageInfo info)
{
	// ...
}
```

## Full list

<!-- vale off -->

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

### 14: `Flash()` {#Flash}

Class: `OnClickFlashRpc`


### 15: `GetBestSpawnPointForPlayer(int flagIDToSpawnOn)` {#GetBestSpawnPointForPlayer}

Class: `PlayerScript`


### 16: `GotKillAssist(float amount, int killedID)` {#GotKillAssist}

Class: `PlayerScript`


### 17: `HealthUpdated(float value)` {#HealthUpdated}

Class: `PlayerScript`


### 18: `InstantiateRpc(int viewID)` {#InstantiateRpc}

Class: `ManualPhotonViewAllocator`


### 19: `JSNow()` {#JSNow}

Class: `PlayerScript`


### 20: `KickPlayer(string playerToKick, string hashedpass)` {#KickPlayer}

Class: `PlayerScript`


### 21: `LatencyReceive(?)` {#LatencyReceive}

Class: `MasterClientFinder`


### 22: `LatencySend(?)` {#LatencySend}

Class: `MasterClientFinder`


### 23: `localCreateGrenade(Vector3 position, Vector3 velocity, float forcedDelay, byte grenadeWeaponType)` {#localCreateGrenade}

Class: `PlayerScript`


### 24: `localHurt(int damagerID, float damage, Vector3 localPosition, byte damagerWeapon, float newHealth)` {#localHurt}

Class: `PlayerScript`


### 25: `localReload()` {#localReload}

Class: `PlayerScript`


### 26: `localSpawnThrowingWeapon(Vector3 position, Vector3 velocity, byte weaponType)` {#localSpawnThrowingWeapon}

Class: `PlayerScript`


### 27: `MapVotedFor(?)` {#MapVotedFor}

### 28: `Marco(?)` {#Marco}

Not found in source code.


### 29: `MatchOverChanged(bool value)` {#MatchOverChanged}

Class: `PlayerScript`


### 30: `mpMeleeAnimation()` {#mpMeleeAnimation}

Class: `PlayerScript`


### 31: `mpThrowGrenadeAnimation()` {#mpThrowGrenadeAnimation}

Class: `PlayerScript`


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

The game will occasionally send out an ad through this RPC call: `RpcSendChatMessage("[Bullet Force]", "Consider supporting the game by buying something in the shop!", 255i16, 255i16, 0i16)`. This message is sent by a player, likely the game's host.


### 51: `RpcShoot(int actorID, float damage, Vector3 position, Vector3 direction, byte numberOfBullets, byte spread, double timeShot, int weaponType)` {#RpcShoot}

Class: `PlayerScript`


### 52: `RpcShowHitmarker()` {#RpcShowHitmarker}

Class: `PlayerScript`


### 53: `RpcShowPerkMessage(string msgUsername, string msg)` {#RpcShowPerkMessage}

Example: `RpcShowPerkMessage("PlayerName", " used Counter UAV")`

Class: `PlayerScript`


### 54: `SetElevatorsClosed(?)` {#SetElevatorsClosed}

### 55: `SetMaps(?)` {#SetMaps}

### 56: `SetNextMap(?)` {#SetNextMap}

### 57: `SetPing(short p)` {#SetPing}

Class: `PlayerScript`


### 58: `SetRank(byte r)` {#SetRank}

Changes the player's rank in the player list.

Class: `PlayerScript`


### 59: `SetSpawnPoint(Vector3 s, int spawnedPlayerActorNr)` {#SetSpawnPoint}

Class: `PlayerScript`


### 60: `SetTimeScale(float s)` {#SetTimeScale}

Supposedly sets the time scale.

When executed as a fresh joined player, this either crashes the lobby or kicks the player (to be checked).

Class: `PlayerScript`


### 61: `ShowAnnouncement(string text, float time)` {#ShowAnnouncement}

Shows a red piece of text on-screen for all players. You do not need to be authenticated for this RPC call, but you need to have instantiated a `PlayerBody`.

Class: `PlayerScript`


### 62: `ShowDebugCapsule(Vector3 pos)` {#ShowDebugCapsule}

Class: `PlayerScript`


### 63: `SpawnFailed(?)` {#SpawnFailed}

### 64: `TaggedPlayer(?)` {#TaggedPlayer}

### 65: `TeleportToPosition(Vector3 position)` {#TeleportToPosition}

Class: `PlayerScript`


### 66: `UpdateAlivePlayers(int _team0Alive, int _team1Alive)` {#UpdateAlivePlayers}

Class: `MatchManager`


### 67: `UpdateHMFFARounds(int playerID, int roundsWon)` {#UpdateHMFFARounds}

Class: `PlayerScript`


### 68: `UpdateMPDeaths(int value)` {#UpdateMPDeaths}

Class: `PlayerScript`


### 69: `UpdateMPKills(int value)` {#UpdateMPKills}

Class: `PlayerScript`


### 70: `UpdateMPRounds(int value)` {#UpdateMPRounds}

Class: `PlayerScript`


### 71: `UpdateTeamNumber(byte value)` {#UpdateTeamNumber}

Class: `ManualPhotonViewAllocator`


### 72: `UpdateTeamPoints(?)` {#UpdateTeamPoints}

### 73: `UpdateTimeInMatch(?)` {#UpdateTimeInMatch}

### 74: `UpdateVIPsOnSubordinates(?)` {#UpdateVIPsOnSubordinates}

### 75: `UsernameChanged(string value)` {#UsernameChanged}

Class: `PlayerScript`


### 76: `WeaponCamoChanged(int value)` {#WeaponCamoChanged}

Class: `PlayerScript`


### 77: `WeaponTypeChanged(byte value)` {#WeaponTypeChanged}

Class: `PlayerScript`


### 78: `RpcACKill(?)` {#RpcACKill}

### 79: `RpcForceKillstreak(KillstreakManager.Killstreak k, bool isOnTheSameTeam)` {#RpcForceKillstreak}

The first argument is most likely sent as an `int`, as that's the default type for enums in C#.

```cs
public enum Killstreak
{
	None = 0,
	UAV = 1,
	SuperSoldier = 2,
	CounterUAV = 3,
	AdvancedUAV = 4,
	Haste = 5,
	Nuke = 6,
}
```

Class: `PlayerScript`


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

If this method isn't called, the player gets kicked from the game. If this gets called with an invalid auth token such as for example an unknown token, a token from another player or when a newer token has generated, then the player also gets kicked from the game.


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

Example: `RpcSendReceiveCustomMapToServer("53779")`


### 107: `RpcCustomMapLikeDislike(?)` {#RpcCustomMapLikeDislike}

<!-- vale on -->
