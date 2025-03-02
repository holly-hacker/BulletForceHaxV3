/// <reference types="../../../bfh3-browser-implant/index" />

import { DevtoolsMessage } from "../../communication";
import * as Base64 from "base64-js";
import { messageTypeNumToString } from "../../util";

declare global {
	interface Window {
		sendMessageToDevtools?: (msg: wasm_bindgen.DevtoolsMessage) => void;
	}
}

export default function () {
	window.sendMessageToDevtools = (msg: wasm_bindgen.DevtoolsMessage) => {
		const copiedMsg: DevtoolsMessage = {
			direction: msg.direction ? "recv" : "send",
			socketType: msg.socket_type ? "game" : "lobby",
			messageType: messageTypeNumToString(msg.message_type),
			message: Base64.fromByteArray(msg.message),
			parsedMessage: msg.parsed_message && Base64.fromByteArray(msg.parsed_message),
			error: msg.error
		};
		msg.free();
		window.postMessage(copiedMsg, "*");
	};
}
