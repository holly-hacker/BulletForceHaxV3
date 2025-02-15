'use strict';

import { GetPatchedFileRequest, GetPatchedFileResponse, AnyRequest, GET_PATCHED_FILE } from "./communication";
import { log } from "./util";
import { fetchOrGetCached } from "./util/fileCache";

chrome.runtime.onMessage.addListener(onMessage);
chrome.runtime.onMessageExternal.addListener(onMessage);

// TODO: pass if external message or not. may be security issue.
function onMessage(request: AnyRequest, sender: chrome.runtime.MessageSender, sendResponse: (response?: any) => void) {
	log(`incoming request from ${sender.url}`, request);

	switch (request.type) {
		case GET_PATCHED_FILE: {
			handleGetPatchedFile(request.data).then(sendResponse);
			return true; // true means sending a response later
		}
	}
}

async function handleGetPatchedFile(request: GetPatchedFileRequest): Promise<GetPatchedFileResponse> {
	let bytes = await fetchOrGetCached(request.url);
	log("Fetched", request.url);

	let js = new TextDecoder().decode(bytes);

	switch (request.role) {
		case 'FRAMEWORK': {
			log("patching framework");

			// disable game logging
			js = js.replace(
				`_JS_Log_Dump(ptr,type){`,
				`_JS_Log_Dump(ptr,type){return;`
			);

			// patch `_SocketCreate` to get notified of opened connections
			js = js.replace(
				`socket.socket.binaryType="arraybuffer";`,
				`socket.socket.binaryType="arraybuffer";` +
				`wasm_bindgen.on_ws_open(str);`
			);

			// patch `_SocketSend` to modify messages as they are sent by the game
			js = js.replace(
				`socket.socket.send(HEAPU8.buffer.slice(ptr,ptr+length))`,
				`let __message = HEAPU8.buffer.slice(ptr,ptr+length);` +
				`wasm_bindgen.on_ws_send(new Uint8Array(__message), socket.socket.url)` +
				`.then((array) => array && socket.socket.send(array))`
			);

			// patch `socket.socket.onmessage` callback to modify messages as they are received
			js = js.replace(
				`socket.messages.push(array)`,
				`wasm_bindgen.on_ws_recv(array, e.origin)` +
				`.then((array) => array && socket.messages.push(array))`
			);

			// patch `socket.socket.onclose` to get notified of closed connections
			js = js.replace(
				`socket.socket.onclose=function(e){`,
				`socket.socket.onclose=function(e){` +
				`wasm_bindgen.on_ws_close(e.currentTarget.url);`
			);
			break;
		}
	}

	return { js };
}

