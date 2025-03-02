'use strict';

import { GetPatchedFileRequest, GetPatchedFileResponse, GET_PATCHED_FILE, isAnyRequest } from "./communication";
import { log, logError } from "./util";
import { fetchOrGetCached } from "./util/fileCache";

log('background service started');

chrome.sidePanel.setPanelBehavior({ openPanelOnActionClick: true }).catch(logError);
chrome.tabs.onUpdated.addListener((...args) => { onTabUpdated(...args).catch(logError); });

async function onTabUpdated(tabId: number, _info: chrome.tabs.TabChangeInfo, tab: chrome.tabs.Tab) {
	if (!tab.url) return;
	const url = new URL(tab.url);

	const isCrazyGames = url.hostname === 'crazygames.com' && url.pathname === '/game/bullet-force';
	const isCrazyGamesWww = url.hostname === 'www.crazygames.com' && url.pathname === '/game/bullet-force';
	const isCgTopFrame = url.hostname === 'games.crazygames.com' && url.pathname.indexOf('bullet-force') !== -1;

	if (isCrazyGames || isCrazyGamesWww || isCgTopFrame) {
		log('Enabling side panel for', tab.url);
		await chrome.action.enable(tabId);
		await chrome.action.setBadgeText({ text: "✔️", tabId });
		await chrome.sidePanel.setOptions({
			tabId,
			path: 'sidepanel.html',
			enabled: true,
		});
	} else {
		await chrome.action.disable(tabId);
		await chrome.action.setBadgeText({ text: "", tabId });
		await chrome.sidePanel.setOptions({
			tabId,
			enabled: false,
		});
	}
}

chrome.runtime.onMessage.addListener((...args) => { onMessage(...args).catch(logError); });
chrome.runtime.onMessageExternal.addListener((...args) => { onMessage(...args).catch(logError); });

// TODO: pass if external message or not. may be security issue.
async function onMessage(request: unknown, sender: chrome.runtime.MessageSender, sendResponse: (response?: unknown) => void) {
	if (!isAnyRequest(request)) return;

	switch (request.type) {
		case GET_PATCHED_FILE: {
			log(`incoming patched file request from ${sender.url}`, request);
			const response = await handleGetPatchedFile(request.data);
			sendResponse(response);
			return true; // true means sending a response later
		}
		default: {
			sendResponse(undefined);
			return;
		}
	}
}

async function handleGetPatchedFile(request: GetPatchedFileRequest): Promise<GetPatchedFileResponse> {
	const bytes = await fetchOrGetCached(request.url);
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

