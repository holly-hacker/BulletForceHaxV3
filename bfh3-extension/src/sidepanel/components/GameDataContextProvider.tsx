import * as React from "react";
import { createContext, useEffect, useState } from "react";
import { LobbyOrGameData, SEND_LOBBY_OR_GAME_DATA } from "../../communication/to_sidepanel";
import { isAnyRequest } from "../../communication";

export const GameDataContext = createContext<LobbyOrGameData | null>(null);

function registerMessageHandler(cb: (msg: LobbyOrGameData) => void): () => void {
	chrome.runtime.onMessage.addListener(onMessage);

	function onMessage(request: unknown, _sender: chrome.runtime.MessageSender, sendResponse: (response?: unknown) => void) {
		if (!isAnyRequest(request)) return;

		switch (request.type) {
			case SEND_LOBBY_OR_GAME_DATA: {
				cb(request.data);
				sendResponse(undefined);
				break;
			}
			default: {
				sendResponse(undefined);
				break;
			}
		}
	}

	return () => chrome.runtime.onMessage.removeListener(onMessage);
}

export default function GameDataContextProvider({ children }: { children?: React.ReactNode }) {
	const [gameData, setGameData] = useState<LobbyOrGameData | null>(null);

	useEffect(() => {
		return registerMessageHandler((msg) => { setGameData(msg); });
	}, []);

	return (
		<GameDataContext.Provider value={gameData}>
			{children}
		</GameDataContext.Provider>
	);
}
