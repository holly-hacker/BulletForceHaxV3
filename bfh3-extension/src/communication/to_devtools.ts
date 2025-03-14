import { chromeRuntimeSend } from ".";
import { MessageTypeString } from "../util";

export const SEND_DEVTOOLS_MESSAGE = 'SendDevtoolsMessage';

export interface DevtoolsMessage {
	/** The direction the packet was going */
	direction: "send" | "recv";
	/** Which server the socket is connected to */
	socketType: "lobby" | "game";
	/** The type of the message */
	messageType: MessageTypeString | null;
	/** The raw message, encoded as MessagePack */
	message: string;
	/** The high-level message (if any), encoded as MessagePack */
	parsedMessage?: string;
	/** The parsing error, if any occurred */
	error?: string;
}

export async function sendDevtoolsMessage(request: DevtoolsMessage, extensionId?: string): Promise<void> {
	await chromeRuntimeSend(
		{ type: SEND_DEVTOOLS_MESSAGE, data: request },
		extensionId
	);
}
