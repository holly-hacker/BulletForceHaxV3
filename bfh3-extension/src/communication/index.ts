import { MessageTypeString } from "../util";

const GET_PATCHED_FILE = 'GetPatchedFile';
const SEND_DEVTOOLS_MESSAGE = 'SendDevtoolsMessage';

type AnyRequest =
	| { type: typeof GET_PATCHED_FILE, data: GetPatchedFileRequest }
	| { type: typeof SEND_DEVTOOLS_MESSAGE, data: DevtoolsMessage };

interface GetPatchedFileRequest {
	url: string;
	role: 'FRAMEWORK';
}

interface GetPatchedFileResponse {
	js: string;
}

/**
 * Request a patched game file URL from the background script
 */
async function getPatchedFile(request: GetPatchedFileRequest, extensionId?: string): Promise<GetPatchedFileResponse> {
	return await chromeRuntimeSend(
		{ type: GET_PATCHED_FILE, data: request },
		extensionId
	) as GetPatchedFileResponse;
}

interface DevtoolsMessage {
	/** The direction the packet was going */
	direction: "send" | "recv";
	/** Which server the socket is connected to */
	socketType: "lobby" | "game";
	/** The type of the message */
	messageType: MessageTypeString | null;
	/** The raw message, encoded as MessagePack */
	message: string;
	/** The high-level message (if any), encoded as MessagePack */
	parsedMessage?: string;
	/** The parsing error, if any occurred */
	error?: string;

}

async function sendDevtoolsMessage(request: DevtoolsMessage, extensionId?: string): Promise<void> {
	await chromeRuntimeSend(
		{ type: SEND_DEVTOOLS_MESSAGE, data: request },
		extensionId
	);
}

function chromeRuntimeSend(request: AnyRequest, extensionId: string | undefined): Promise<unknown> {
	return new Promise((resolve, reject) => {
		try {
			chrome.runtime.sendMessage(extensionId, request, response => resolve(response));
		} catch (e) {
			reject(e);
		}
	});
}

export {
	AnyRequest,
	getPatchedFile, GET_PATCHED_FILE, GetPatchedFileRequest, GetPatchedFileResponse,
	sendDevtoolsMessage, SEND_DEVTOOLS_MESSAGE, DevtoolsMessage,
}
