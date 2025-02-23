'use strict';

import { DevtoolsMessage, sendDevtoolsMessage } from "../communication";
import { log, logError, onDomLoaded } from "../util";

log("Running game frame document_start script");

// add our loadGameHook script
onDomLoaded(() => {
	const runtimeUrl = chrome.runtime.getURL("webAccessibleResources_gameFrameRuntime.js");
	const runtimeScriptNode = document.createElement("script");
	runtimeScriptNode.src = runtimeUrl;
	document.body.append(runtimeScriptNode);

	const wasmJsUrl = chrome.runtime.getURL("wasm/index.js");
	const wasmJsScriptNode = document.createElement("script");
	wasmJsScriptNode.src = wasmJsUrl;
	document.body.append(wasmJsScriptNode);

	const wasmInitUrl = chrome.runtime.getURL("init_wasm_module.js");
	const wasmInitScriptNode = document.createElement("script");
	wasmInitScriptNode.src = wasmInitUrl;
	document.body.append(wasmInitScriptNode);
});

var mo = new MutationObserver((mutationList, observer) => {
	for (const record of mutationList) {
		if (record.type !== "childList") continue;

		for (const node of record.addedNodes) {
			if (node.nodeName != "SCRIPT") continue;
			const scriptNode = node as HTMLScriptElement;
			if (scriptNode.src) continue;
			// we want specifically the main script tag that accepts the message from the parent frame
			if (!scriptNode.textContent || scriptNode.textContent.indexOf('sendUnityMessage') == -1) continue;

			// log("original script content", scriptNode.textContent);

			if (scriptNode.textContent.indexOf("window.loadGame()") === -1) {
				logError("Could not find window.loadGame() string in js");
				continue;
			}

			// we need to pass in the extension id so chrome.runtime.sendMessage knows which extension to contact
			let extensionId = chrome.runtime.id;
			log("patching loadGame call to loadGameHook with extension id", extensionId);
			scriptNode.textContent = scriptNode.textContent.replace("window.loadGame()", `window.loadGameHook('${extensionId}')`);

			// we no longer need the mutation observer
			observer.disconnect();
		}
	}
});

mo.observe(document.getRootNode(), {
	childList: true,
	subtree: true,
});

// install some event listener to get messages to dispatch onwards
window.addEventListener('message', async (event) => {
	// Only accept messages from the same frame, not from the parent
	if (event.source !== window) return;

	const data = event.data;

	if (typeof data !== 'object') return;

	if (data?.message) {
		// probably devtools message. could be a better check.
		const msg = data as DevtoolsMessage;

		await sendDevtoolsMessage(msg);
	}
});
