import { chromeRuntimeSend } from ".";

export const GET_PATCHED_FILE = 'GetPatchedFile';

export interface GetPatchedFileRequest {
	url: string;
	role: 'FRAMEWORK';
}

export interface GetPatchedFileResponse {
	js: string;
}

/**
 * Request a patched game file URL from the background script
 */
export async function getPatchedFile(request: GetPatchedFileRequest, extensionId?: string): Promise<GetPatchedFileResponse> {
	return await chromeRuntimeSend(
		{ type: GET_PATCHED_FILE, data: request },
		extensionId
	) as GetPatchedFileResponse;
}
