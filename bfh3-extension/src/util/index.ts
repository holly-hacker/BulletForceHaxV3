const log = function (...data: any) {
	console.log("%c[BulletForceHaxV3]%c", "color: hotpink", "color: initial", ...data);
};
const logError = function (...data: any) {
	console.error("%c[BulletForceHaxV3]%c", "color: hotpink", "color: initial", ...data);
};

function onDomLoaded(callback: () => void) {
	if (document.readyState === 'loading') {
		document.addEventListener('DOMContentLoaded', function () {
			callback();
		});
	} else {  // 'interactive' or 'complete'
		callback();
	}
}

export { log, logError, onDomLoaded };
