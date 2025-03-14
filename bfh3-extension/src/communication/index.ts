import { toError } from "../util";
import { GET_PATCHED_FILE, GetPatchedFileRequest } from "./to_background";
import { DevtoolsMessage, SEND_DEVTOOLS_MESSAGE } from "./to_devtools";

export type AnyRequest =
	| { type: typeof GET_PATCHED_FILE, data: GetPatchedFileRequest }
	| { type: typeof SEND_DEVTOOLS_MESSAGE, data: DevtoolsMessage };

export function isAnyRequest(message: unknown): message is AnyRequest {
	if (!(message && typeof message === 'object')) return false;
	if (!('type' in message && typeof message.type === 'string')) return false;
	if (!('data' in message)) return false;

	return true;
}

export function chromeRuntimeSend(request: AnyRequest, extensionId: string | undefined): Promise<unknown> {
	return new Promise((resolve, reject) => {
		try {
			chrome.runtime.sendMessage(extensionId, request, response => resolve(response));
		} catch (e) {
			reject(toError(e));
		}
	});
}
