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

		{selectedMessage.detail && <>
			<h2>{selectedMessage.hasError ? 'Error' : 'Detail'}</h2>
			<pre>{selectedMessage.detail}</pre>
		</>}

		{selectedMessage.interpretedMessage && <>
			<h2>Interpreted message</h2>
			<pre>{JSON.stringify(selectedMessage.interpretedMessage, null, 4)}</pre>
		</>}

		<h2>Parsed message</h2>
		<pre>{selectedMessage.liftedMessage
			? JSON.stringify(selectedMessage.liftedMessage, null, 4)
			: '<none>'
		}</pre>

		<h2>Raw message</h2>
		<pre>{JSON.stringify(selectedMessage.rawMessage, null, 4)}</pre>
	</div>;
};
