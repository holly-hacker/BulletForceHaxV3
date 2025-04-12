import * as React from "react";
import TopPanel from "./TopPanel";
import MessageListWithSidePanel from "./MessageListWithSidePanel";
import { UnpackedDevtoolsMessage } from "./MessageList";
import { DevtoolsMessage, SEND_DEVTOOLS_MESSAGE } from "../../communication/to_devtools";
import { isAnyRequest } from "../../communication";
import { useEffect, useState } from "react";

function registerMessageHandler(cb: (msg: DevtoolsMessage) => void): () => void {
	chrome.runtime.onMessage.addListener(onMessage);

	function onMessage(request: unknown, _sender: chrome.runtime.MessageSender, sendResponse: (response?: unknown) => void) {
		if (!isAnyRequest(request)) return;
		// log(`incoming request from ${sender.url}`, request);

		if (request.type == SEND_DEVTOOLS_MESSAGE) {
			cb(request.data);
		}

		sendResponse(undefined);
	}

	return () => chrome.runtime.onMessage.removeListener(onMessage);
}


export default function DevtoolsTab() {
	const topTabHeight = 48;

	const [messages, setMessages] = useState<UnpackedDevtoolsMessage[]>([]);

	useEffect(() => {
		return registerMessageHandler((msg) => {
			let rawMessage, liftedMessage, interpretedMessage, detail, hasError = false;
			try {
				detail = msg.detail;
				hasError = msg.hasError;

				// ops that can error, in order of dependency
				rawMessage = JSON.parse(msg.rawMessage) as object;
				liftedMessage = msg.liftedMessage ? JSON.parse(msg.liftedMessage) as object : undefined;
				interpretedMessage = msg.interpretedMessage ? JSON.parse(msg.interpretedMessage) as object : undefined;
			} catch (e) {
				if (!e)
					detail = undefined;
				else if (e instanceof Error)
					detail = e.name;
				else if (typeof e === 'string')
					detail = e;
				else
					detail = JSON.stringify(e);
			}
			const unpackedMessage: UnpackedDevtoolsMessage = {
				direction: msg.direction,
				socketType: msg.socketType,
				messageType: msg.messageType,
				rawMessage,
				liftedMessage,
				interpretedMessage,
				detail,
				hasError: hasError,
			};
			setMessages(prevMessages => [...prevMessages, unpackedMessage]);
		});
	}, []);

	return (
		<div style={{ height: '100vh' }}>
			<div style={{ height: `${topTabHeight}px`, padding: '4px' }}>
				<TopPanel onClear={() => setMessages([])} />
			</div>
			<div style={{ height: `calc(100% - ${topTabHeight}px)`, overflow: 'auto', }}>
				<MessageListWithSidePanel messages={messages} />
			</div>
		</div>
	);
}
