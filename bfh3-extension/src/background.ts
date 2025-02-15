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
			js = js.replace(
				"socket.messages=socket.messages.slice(1)",
				`console.log('Sent socket message', socket.messages[0]);` +
				"socket.messages=socket.messages.slice(1)");
			break;
		}
	}

	return { js };
}

