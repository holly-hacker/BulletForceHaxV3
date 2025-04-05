import * as React from "react";
import { UnpackedDevtoolsMessage } from "./MessageList";

export default function SidePanel({ selectedMessage, onClose }: { selectedMessage: UnpackedDevtoolsMessage, onClose: () => void }) {
	return <div>
		<button onClick={onClose}>Close</button>

		<ul>
			<li>Direction: <code>{selectedMessage.direction}</code></li>
			<li>SocketType: <code>{selectedMessage.socketType}</code></li>
			<li>MessageType: <code>{selectedMessage.messageType}</code></li>
		</ul>

		{selectedMessage.error && <>
			<h2>Error</h2>
			<pre>{selectedMessage.error}</pre>
		</>}

		<h2>Parsed message</h2>
		<pre>{selectedMessage.parsedMessage
			? JSON.stringify(selectedMessage.parsedMessage, null, 4)
			: '<none>'
		}</pre>

		<h2>Raw message</h2>
		<pre>{JSON.stringify(selectedMessage.message, null, 4)}</pre>
	</div>;
};
