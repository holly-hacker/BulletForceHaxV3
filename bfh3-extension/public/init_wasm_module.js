/* eslint-disable @typescript-eslint/restrict-plus-operands -- only for error cases */

// TODO: this timeout is a hack. wasm_bindgen depends on its js module to be loaded. Without the timeout, it seems to
// be a 50/50 race condition.
setTimeout(() => {
	try {
		// eslint-disable-next-line no-undef
		wasm_bindgen()
			.then(module => {
				// set up tick to be called every 100ms
				setInterval(() => { module.tick(); }, 100);
			})
			// eslint-disable-next-line @typescript-eslint/use-unknown-in-catch-callback-variable -- not a typescript file
			.catch(r => alert("wasm_bindgen call failed: " + r));
	} catch (e) {
		console.error("failed to load bfh wasm bundle:", e);
		alert("failed to load bfh wasm bundle: " + e);
	}
}, 100);
