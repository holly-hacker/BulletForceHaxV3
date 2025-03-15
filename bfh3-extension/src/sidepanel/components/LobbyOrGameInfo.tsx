import * as React from "react";
import { GameDataContext } from "./GameDataContextProvider";
import { useContext } from "react";
import { GAME_DATA, LOBBY_DATA } from "../../communication/to_sidepanel";
import GameInfo from "./GameInfo";
import LobbyInfo from "./LobbyInfo";

export default function LobbyOrGameInfo() {
	const gameData = useContext(GameDataContext);
	if (!gameData) {
		return (<>No socket connection...</>);
	}
	switch (gameData.type) {
		case LOBBY_DATA: {
			return <LobbyInfo data={gameData.data} />;
		}
		case GAME_DATA: {
			return <GameInfo />;
		}
	}
}
