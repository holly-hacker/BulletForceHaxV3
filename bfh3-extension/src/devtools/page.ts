import { isGameHostUrl, log } from "../util";

chrome.devtools.inspectedWindow.eval('document.URL', {}, (result: string) => {
	const url = URL.parse(result);

	if (!url) return;

	if (isGameHostUrl(url)) {
		chrome.devtools.panels.create("BulletForceHaxV3", "icons/icon_32.png", "devtools_panel.html", () => {
			log("enabled devtools pane");
		})
	}
});
