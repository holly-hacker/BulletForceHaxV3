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

/**
 * Request a patched game file URL from the background script
 */
async function getPatchedFile(request: GetPatchedFileRequest, extensionId?: string): Promise<GetPatchedFileResponse> {
	return await chromeRuntimeSend(
		{ type: GET_PATCHED_FILE, data: request },
		extensionId
	);
}

function chromeRuntimeSend(request: AnyRequest, extensionId: string | undefined): Promise<any> {
	return new Promise((resolve) => {
		chrome.runtime.sendMessage(extensionId, request, response => resolve(response));
	});
}

export {
	AnyRequest,
	getPatchedFile, GET_PATCHED_FILE, GetPatchedFileRequest, GetPatchedFileResponse,
}
