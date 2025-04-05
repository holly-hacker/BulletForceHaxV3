/// <reference types="../../../bfh3-browser-implant/index" />

import { messageTypeNumToString } from "../../util";
import { DevtoolsMessage } from "../../communication/to_devtools";
import { GAME_DATA, LOBBY_DATA, LobbyData, LobbyMatch, LobbyOrGameData } from "../../communication/to_sidepanel";

declare global {
	interface Window {
		sendMessageToDevtools?: (msg: wasm_bindgen.DevtoolsMessage) => void;
		sendLobbyData?: (msg: wasm_bindgen.LobbyData) => void;
		sendGameData?: (msg: wasm_bindgen.GameData) => void;
	}
}

export default function () {
	window.sendMessageToDevtools = (msg: wasm_bindgen.DevtoolsMessage) => {
		const copiedMsg: DevtoolsMessage = {
			direction: msg.direction ? "recv" : "send",
			socketType: msg.socket_type ? "game" : "lobby",
			messageType: messageTypeNumToString(msg.message_type),
			rawMessage: msg.raw_message,
			liftedMessage: msg.lifted_message,
			interpretedMessage: msg.interpreted_message,
			hasError: msg.has_error,
			detail: msg.detail
		};
		msg.free();
		window.postMessage(copiedMsg, "*");
	};

	window.sendLobbyData = (msg: wasm_bindgen.LobbyData) => {
		const matches: LobbyMatch[] = msg.matches.map(match => {
			return {
				key: match.key,
				players: match.players,
				max_players: match.max_players,
				name: match.name,
				map: match.map,
				platform: match.platform,
				version: match.version,
			};
		});

		const lobbyData: LobbyData = {
			matches: matches
		};

		const lobbyOrGameData: LobbyOrGameData = { type: LOBBY_DATA, data: lobbyData, };

		window.postMessage(lobbyOrGameData, "*");

		msg.free();
	}

	window.sendGameData = (msg: wasm_bindgen.GameData) => {
		const lobbyOrGameData: LobbyOrGameData = { type: GAME_DATA, data: {}, };

		window.postMessage(lobbyOrGameData, "*");

		msg.free();
	}
}
