import { log } from ".";
import * as Base64 from "base64-js";

async function fetchOrGetCached(url: string): Promise<Uint8Array> {
	type StoreType = { [key: string]: CachedFile };

	const fileKey = `file-${url}`;
	const storageResult = await chrome.storage.local.get<StoreType>([fileKey]);
	const cachedItem = storageResult[fileKey];
	if (cachedItem) {
		// file is cached, but not sure if up-to-date
		log("Already found", url, "in local storage, checking if up to date");

		const headResponse = await fetch(url, { method: "HEAD" });
		const etag = headResponse.headers.get("ETag");
		const lastModified = headResponse.headers.get("Last-Modified");
		log("Header values in cache:", etag, lastModified);

		const storedIsUpToDate = etag == cachedItem.etag && lastModified == cachedItem.lastModified;
		if (storedIsUpToDate) {
			log("File was up to date, no need to re-fetch");
			return Base64.toByteArray(cachedItem.dataBase64);
		}
	}

	// file is not yet cached or is out-of-date, fetch it
	log("file not yet cached, fetching again");
	const response = await fetch(url);
	const etag = response.headers.get("ETag")
	const lastModified = response.headers.get("Last-Modified");

	log("Header values after fetch", etag, lastModified);
	const buffer = new Uint8Array(await response.arrayBuffer());
	log("response size", buffer.byteLength);
	const bufferBase64 = Base64.fromByteArray(buffer);

	let toStore: StoreType = {};
	toStore[fileKey] = {
		url: url,
		etag: etag,
		lastModified: lastModified,
		dataBase64: bufferBase64,
	};
	await chrome.storage.local.set<StoreType>(toStore);

	return buffer;
}

interface CachedFile {
	url: string,
	etag: string | null,
	lastModified: string | null,
	dataBase64: string,
}

export { fetchOrGetCached }
