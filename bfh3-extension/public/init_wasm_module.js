// TODO: this timeout is a hack. wasm_bindgen depends on its js module to be loaded. Without the timeout, it seems to
// be a 50/50 race condition.
setTimeout(() => {
	try {
		// eslint-disable-next-line no-undef
		wasm_bindgen();
	} catch (e) {
		console.error("failed to load bfh wasm bundle:",e);
		alert("failed to load bfh wasm bundle: " + e);
	}
}, 100);
