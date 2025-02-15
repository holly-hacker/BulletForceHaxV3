const GET_PATCHED_FILE = 'GetPatchedFile';

type AnyRequest =
	| { type: typeof GET_PATCHED_FILE, data: GetPatchedFileRequest };

interface GetPatchedFileRequest {
	url: string;
	role: 'FRAMEWORK';
}

interface GetPatchedFileResponse {
	js: string;
}

function getPatchedFile(request: GetPatchedFileRequest, callback: (response: GetPatchedFileResponse) => void, extensionId?: string) {
	doSend({ type: GET_PATCHED_FILE, data: request }, callback, extensionId);
}

function doSend(request: AnyRequest, callback: (response: any) => void, extensionId: string | undefined) {
	chrome.runtime.sendMessage(extensionId, request, callback);
}

export {
	AnyRequest,
	getPatchedFile, GET_PATCHED_FILE, GetPatchedFileRequest, GetPatchedFileResponse,
}
