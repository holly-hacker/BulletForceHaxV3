// TODO: this timeout is a hack. wasm_bindgen depends on its js module to be loaded. Without the timeout, it seems to
// be a 50/50 race condition.
setTimeout(() => {
	try {
		wasm_bindgen();
	} catch (e) {
		alert("failed to load bfh wasm bundle");
	}
}, 100);
