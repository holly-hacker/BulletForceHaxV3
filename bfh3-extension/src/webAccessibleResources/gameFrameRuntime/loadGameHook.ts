import { getPatchedFile } from "../../communication/to_background";
import { log, logError } from "../../util";

// can't put these in a `.d.ts` file?
declare global {
	interface Window {
		/** The original loadGame function */
		loadGame: () => void;
		/** The loadGame hook function created in this script */
		loadGameHook?: (extensionId: string) => void | Promise<void>;
		/** The unity config object */
		config: UnityConfig | Unity2020Config;
	}
}

interface UnityConfig {
	type: Exclude<string, typeof Unity2020ConfigType>,
}

const Unity2020ConfigType = 'unity2020config';
interface Unity2020Config {
	type: typeof Unity2020ConfigType,
	loaderOptions: {
		showProgress: boolean,
		unityConfigOptions: {
			codeUrl: string,
			dataUrl: string,
			frameworkUrl: string,
			streamingAssetsUrl: string,
		},
		unityLoaderUrl: string,
	},
	oldSdkInitObject: {
		gameLink: string,
		userInfo: unknown,
	},
	unityConfig: {
		companyName: string,
		productName: string,
		productVersion: string,

		codeUrl: string,
		dataUrl: string,
		frameworkUrl: string,
		streamingAssetsUrl: string,
	},
	unitySaveFileNames: unknown,
}

export default function () {
	window.loadGameHook = async function (extensionId: string) {
		if (window.config.type !== Unity2020ConfigType) {
			logError(`config type is not ${Unity2020ConfigType}, but is instead`, window.config.type)
			return;
		}

		const unityConfig = window.config as Unity2020Config; // type system is not smart enough to recognize this

		// get the patched loader, put it in the loader object and call the original loadGame function
		// this function should have been updated to load the `src` function
		const { js } = await getPatchedFile(
			{ url: unityConfig.loaderOptions.unityConfigOptions.frameworkUrl, role: 'FRAMEWORK' },
			extensionId
		);

		const blob = new Blob([js], { type: "application/javascript" });
		const blobUrl = URL.createObjectURL(blob);
		unityConfig.loaderOptions.unityConfigOptions.frameworkUrl = blobUrl;
		unityConfig.unityConfig.frameworkUrl = blobUrl;
		log("Patched framework url to", blobUrl);

		window.loadGame();
	};
}
