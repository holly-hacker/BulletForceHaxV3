export const log = function (...data: unknown[]) {
	console.log("%c[BulletForceHaxV3]%c", "color: hotpink", "color: initial", ...data);
};

export const logError = function (...data: unknown[]) {
	console.error("%c[BulletForceHaxV3]%c", "color: hotpink", "color: initial", ...data);
};

export function onDomLoaded(callback: () => void) {
	if (document.readyState === 'loading') {
		document.addEventListener('DOMContentLoaded', function () {
			callback();
		});
	} else {  // 'interactive' or 'complete'
		callback();
	}
}

export type MessageTypeString =
	| 'Init'
	| 'InitResponse'
	| 'OperationRequest'
	| 'OperationResponse'
	| 'Event'
	| 'Disconnect'
	| 'InternalOperationRequest'
	| 'InternalOperationResponse'
	| 'Message'
	| 'RawMessage'
	| 'PingResult';

export function messageTypeNumToString(num: number): MessageTypeString  | null{
	switch (num) {
		case 0: return 'Init';
		case 1: return 'InitResponse';
		case 2: return 'OperationRequest';
		case 3: return 'OperationResponse';
		case 4: return 'Event';
		case 5: return 'Disconnect';
		case 6: return 'InternalOperationRequest';
		case 7: return 'InternalOperationResponse';
		case 8: return 'Message';
		case 9: return 'RawMessage';
		case 10: return 'PingResult';
		default: return null;
	}
}
