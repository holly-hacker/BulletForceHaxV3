import * as React from "react";
import { useEffect, useState } from "react";
import { AnyRequest, DevtoolsMessage, SEND_DEVTOOLS_MESSAGE } from "../../communication";
import { log } from "../../util";
import * as Msgpack from "@msgpack/msgpack";
import * as Base64 from "base64-js";
import { createColumnHelper, flexRender, getCoreRowModel, RowModel, Table, useReactTable } from "@tanstack/react-table";

interface UnpackedDevtoolsMessage {
	/** The direction the packet was going */
	direction: "send" | "recv";
	/** Which server the socket is connected to */
	socketType: "lobby" | "game";
	/** The type of the message */
	messageType: number;
	/** The raw message */
	message?: Object;
	/** The high-level message (if any) */
	parsedMessage?: Object;
	/** The parsing error, if any occurred */
	error?: string;
}

function registerMessageHandler(cb: (msg: DevtoolsMessage) => void): () => void {
	chrome.runtime.onMessage.addListener(onMessage);

	function onMessage(request: AnyRequest, sender: chrome.runtime.MessageSender, sendResponse: (response?: any) => void) {
		// log(`incoming request from ${sender.url}`, request);

		switch (request.type) {
			case SEND_DEVTOOLS_MESSAGE: {
				cb(request.data);
				sendResponse(undefined);
			}
			default: {
				sendResponse(undefined);
			}
		}
	}

	return () => chrome.runtime.onMessage.removeListener(onMessage);
}

const columnHelper = createColumnHelper<UnpackedDevtoolsMessage>();
const columns = [
	columnHelper.accessor('direction', {
		header: 'Dir',
		cell: info => info.getValue() == 'recv'
		? <span style={({color: 'red'})}>⬇</span>
		: <span style={({color: 'lime'})}>⬆</span>,
	}),
	columnHelper.accessor('socketType', {
		header: 'Socket',
		cell: info => info.getValue() == 'lobby' ? "Lobby" : "Game",
	}),
	columnHelper.accessor('messageType', {
		header: 'Message Type',
		cell: info => messageTypeNumToString(info.getValue()),
	}),
	columnHelper.display({
		id: 'parsedName',
		header: 'Parameter Type',
		cell: props => getParsedName(props.row.original) ?? '<unknown>',
	}),
	columnHelper.accessor('error', {
		header: 'Error',
	}),
];

function messageTypeNumToString(num: number): string {
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
		default: return '<unknown>';
	}
}

function getParsedName(message: UnpackedDevtoolsMessage): String | null {
	// don't do anything for init and initresponse
	if ([0, 1].includes(message.messageType)) return '';

	if (!message.parsedMessage) return null;

	// OperationResponse contains an array as top-level, with first item being the actual parsed message
	const toCheck = Array.isArray(message.parsedMessage) ? message.parsedMessage[0] : message.parsedMessage;

	const keys = Object.keys(toCheck);
	return keys[0] ? keys[0] : null;
}

export default function () {
	let [messages, setMessages] = useState<UnpackedDevtoolsMessage[]>([]);

	useEffect(() => {
		return registerMessageHandler((msg) => {
			let message, parsedMessage, error;
			try	{
				message = Msgpack.decode(Base64.toByteArray(msg.message)) as Object;
				parsedMessage = msg.parsedMessage ? Msgpack.decode(Base64.toByteArray(msg.parsedMessage)) as Object : undefined;
				error = msg.error;
			} catch (e) {
				error = (e as any).toString();
			}
			const unpackedMessage: UnpackedDevtoolsMessage = {
				direction: msg.direction,
				socketType: msg.socketType,
				messageType: msg.messageType,
				message,
				parsedMessage,
				error,
			};
			log("msg in cb", unpackedMessage);
			setMessages(prevMessages => [...prevMessages, unpackedMessage]);
		});
	}, []);

	const table = useReactTable({
		columns,
		data: messages,
		getCoreRowModel: getCoreRowModel()
	});

	return (
		<>
			<table className="devtools-table">
				<thead>
					{table.getHeaderGroups().map(headerGroup => (
						<tr key={headerGroup.id}>
							{headerGroup.headers.map(header => (
								<th key={header.id}>
									{header.isPlaceholder
										? null
										: flexRender(header.column.columnDef.header, header.getContext())}
								</th>
							))}
						</tr>
					))}
				</thead>
				<tbody>
					{table.getRowModel().rows.map(row => (
						<tr key={row.id} className={row.original.error ? "has-error" : ""}>
							{row.getVisibleCells().map(cell => (
								<td key={cell.id}>
									{flexRender(cell.column.columnDef.cell, cell.getContext())}
								</td>
							))}
						</tr>
					))}
				</tbody>
			</table>
		</>
	);
}
