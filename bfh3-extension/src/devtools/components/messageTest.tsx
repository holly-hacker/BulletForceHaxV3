import * as React from "react";
import { useEffect, useState } from "react";
import { AnyRequest, DevtoolsMessage, SEND_DEVTOOLS_MESSAGE } from "../../communication";
import { log } from "../../util";
import * as Msgpack from "@msgpack/msgpack";
import * as Base64 from "base64-js";

function registerMessageHandler(cb: (msg: DevtoolsMessage) => void): () => void {
	chrome.runtime.onMessage.addListener(onMessage);

	function onMessage(request: AnyRequest, sender: chrome.runtime.MessageSender, sendResponse: (response?: any) => void) {
		// log(`incoming request from ${sender.url}`, request);

		switch (request.type) {
			case SEND_DEVTOOLS_MESSAGE: {
				cb(request.data);
				sendResponse(undefined);
			}
			default: {
				sendResponse(undefined);
		}
	}

	return () => chrome.runtime.onMessage.removeListener(onMessage);
}

export default function () {
	let [messages, setMessages] = useState<any[]>([]);
	useEffect(() => {
		registerMessageHandler((msg) => {
			log("msg in cb", msg);
			const decoded = Base64.toByteArray(msg.parsedMessage ?? msg.message);
			const parsed = Msgpack.decode<any>(decoded);
			messages.push(parsed);
			// setMessages(messages);
			setMessages(prevMessages => [...prevMessages, parsed]);
		});
	}, []);

	return (
		<>
			<h3>Messages:</h3>
			{messages.length == 0 && "None"}
			<ul>
				{messages.map((msg, i) => (<li key={i}><pre>{JSON.stringify(msg)}</pre></li>))}
			</ul></>
	);
}
