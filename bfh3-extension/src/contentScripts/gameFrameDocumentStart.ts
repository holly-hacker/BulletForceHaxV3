'use strict';

import { log, logError, onDomLoaded } from "../util";

log("Running game frame document_start script");

// add our loadGameHook script
onDomLoaded(() => {
	const loaddGameHookUrl = chrome.runtime.getURL("webAccessibleResources_gameFrame_loadGameHook.js");
	const scriptNode = document.createElement("script");
	scriptNode.src = loaddGameHookUrl;
	document.body.append(scriptNode);

	const wasmJsUrl = chrome.runtime.getURL("wasm/index.js");
	const scriptNode2 = document.createElement("script");
	scriptNode2.src = wasmJsUrl;
	document.body.append(scriptNode2);

	const wasmInitUrl = chrome.runtime.getURL("init_wasm_module.js");
	const scriptNode3 = document.createElement("script");
	scriptNode3.src = wasmInitUrl;
	document.body.append(scriptNode3);
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
