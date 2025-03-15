import * as React from "react";
import { LobbyData } from "../../communication/to_sidepanel";

export default function LobbyInfo({data}: {data: LobbyData}) {
	return (
		<ul>
			{data.matches.map(m => {
				return (
					<li key={m.key}>
						{m.key} / {m.name}: {m.players}/{m.max_players} (<code>{m.platform}</code>)
					</li>
				);
			})}
		</ul>
	);
}
