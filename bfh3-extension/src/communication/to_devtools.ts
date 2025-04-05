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
	/** The raw message, encoded as JSON */
	rawMessage: string;
	/** The high-level message (if any), encoded as JSON */
	liftedMessage?: string;
	/** An interpreted version of the message (if any), encoded as JSON */
	interpretedMessage?: string;
	/** Whether there was an error parsing or lifting this packet */
	hasError: boolean,
	/** The parsing error, if any occurred */
	detail?: string;
}

export async function sendDevtoolsMessage(request: DevtoolsMessage, extensionId?: string): Promise<void> {
	await chromeRuntimeSend(
		{ type: SEND_DEVTOOLS_MESSAGE, data: request },
		extensionId
	);
}
