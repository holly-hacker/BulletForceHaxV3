import { log } from "../util";

chrome.devtools.inspectedWindow.eval('document.URL', {}, (result: string) => {
	const url = URL.parse(result);

	if (!url) return;

	if (url.host == "games.crazygames.com" && url.pathname == '/en_US/bullet-force-multiplayer/index.html') {
		chrome.devtools.panels.create("BulletForceHaxV3", "icons/icon_32.png", "devtools_panel.html", () => {
			log("enabled devtools pane");
		})
	}
});
