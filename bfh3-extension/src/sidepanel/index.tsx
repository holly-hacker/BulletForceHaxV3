import * as React from 'react';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import GameDataContextProvider from './components/GameDataContextProvider';
import LobbyOrGameInfo from './components/LobbyOrGameInfo';

const rootNode = document.getElementById('root');
if (!rootNode) throw Error("no root node found");

const root = createRoot(rootNode);
root.render((
	<StrictMode>
		<GameDataContextProvider>
			<h1>Hello sidepanel</h1>
			<LobbyOrGameInfo />
		</GameDataContextProvider>
	</StrictMode>
));
