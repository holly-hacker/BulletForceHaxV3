// TODO: this timeout is a hack. wasm_bindgen depends on its js module to be loaded. Without the timeout, it seems to
// be a 50/50 race condition.
setTimeout(() => {
	try {
		// eslint-disable-next-line no-undef
		wasm_bindgen()
			.catch(r => alert("wasm_bindgen call failed: " + r));
	} catch (e) {
		console.error("failed to load bfh wasm bundle:", e);
		// eslint-disable-next-line @typescript-eslint/restrict-plus-operands
		alert("failed to load bfh wasm bundle: " + e);
	}
}, 100);
