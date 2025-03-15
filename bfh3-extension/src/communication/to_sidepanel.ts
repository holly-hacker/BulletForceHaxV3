import { chromeRuntimeSend } from ".";

export const SEND_LOBBY_OR_GAME_DATA = 'SendLobbyOrGameData';
export const LOBBY_DATA = 'Lobby';
export const GAME_DATA = 'Game';

export type LobbyOrGameData =
	| { type: typeof LOBBY_DATA, data: LobbyData }
	| { type: typeof GAME_DATA, data: GameData };

export interface LobbyData {
	matches: LobbyMatch[];
}

export interface LobbyMatch {
	key: string;
	players: number;
	max_players: number;
	name: string;
	map: string;
	platform: string;
	version: string;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type -- filled in later
export interface GameData {}

export async function sendLobbyOrGameData(request: LobbyOrGameData, extensionId?: string): Promise<void> {
	await chromeRuntimeSend(
		{ type: SEND_LOBBY_OR_GAME_DATA, data: request },
		extensionId
	);
}
