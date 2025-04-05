import * as React from "react";
import { useEffect, useState } from "react";
import { isAnyRequest } from "../../communication";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { useVirtualizer } from "@tanstack/react-virtual";
import { MessageTypeString } from "../../util";
import { DevtoolsMessage, SEND_DEVTOOLS_MESSAGE } from "../../communication/to_devtools";

export interface UnpackedDevtoolsMessage {
	/** The direction the packet was going */
	direction: "send" | "recv";
	/** Which server the socket is connected to */
	socketType: "lobby" | "game";
	/** The type of the message */
	messageType: MessageTypeString | null;
	/** The raw message */
	message?: object;
	/** The high-level message (if any) */
	parsedMessage?: object;
	/** The parsing error, if any occurred */
	error?: string;
}

function registerMessageHandler(cb: (msg: DevtoolsMessage) => void): () => void {
	chrome.runtime.onMessage.addListener(onMessage);

	function onMessage(request: unknown, _sender: chrome.runtime.MessageSender, sendResponse: (response?: unknown) => void) {
		if (!isAnyRequest(request)) return;
		// log(`incoming request from ${sender.url}`, request);

		switch (request.type) {
			case SEND_DEVTOOLS_MESSAGE: {
				cb(request.data);
				sendResponse(undefined);
				break;
			}
			default: {
				sendResponse(undefined);
				break;
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
			? <span style={{ color: 'red' }}>⬇</span>
			: <span style={{ color: 'lime' }}>⬆</span>,
	}),
	columnHelper.accessor('socketType', {
		header: 'Socket',
		cell: info => info.getValue() == 'lobby' ? "Lobby" : "Game",
	}),
	columnHelper.accessor('messageType', {
		header: 'Message Type',
		cell: info => info.getValue(),
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

function getParsedName(message: UnpackedDevtoolsMessage): string | null {
	// don't do anything for init and initresponse
	if (!message.messageType) return '';
	if (['Init', 'InitResponse'].includes(message.messageType)) return '';

	if (!message.parsedMessage) return null;

	// OperationResponse contains an array as top-level, with first item being the actual parsed message
	const toCheck = Array.isArray(message.parsedMessage) ? message.parsedMessage[0] as object : message.parsedMessage;

	const keys = Object.keys(toCheck);
	return keys[0] ? keys[0] : null;
}

export default function MessageList({ scrollRef, selectedMessage, onItemSelected }: {
	scrollRef: React.RefObject<HTMLDivElement | null>,
	selectedMessage: UnpackedDevtoolsMessage | null,
	onItemSelected: (a: UnpackedDevtoolsMessage) => void,
}) {
	const [messages, setMessages] = useState<UnpackedDevtoolsMessage[]>([]);

	useEffect(() => {
		return registerMessageHandler((msg) => {
			let message, parsedMessage, error;
			try {
				message = JSON.parse(msg.message) as object;
				parsedMessage = msg.parsedMessage ? JSON.parse(msg.parsedMessage) as object : undefined;
				error = msg.error;
			} catch (e) {
				if (!e)
					error = undefined;
				else if (e instanceof Error)
					error = e.name;
				else if (typeof e === 'string')
					error = e;
				else
					error = JSON.stringify(e);
			}
			const unpackedMessage: UnpackedDevtoolsMessage = {
				direction: msg.direction,
				socketType: msg.socketType,
				messageType: msg.messageType,
				message,
				parsedMessage,
				error,
			};
			setMessages(prevMessages => [...prevMessages, unpackedMessage]);
		});
	}, []);

	const virtualizer = useVirtualizer({
		count: messages.length,
		getScrollElement: () => scrollRef.current,
		estimateSize: (_idx) => 20,
	});

	const table = useReactTable({
		columns,
		data: messages,
		getCoreRowModel: getCoreRowModel()
	});

	const { rows } = table.getRowModel();

	return (
		<div style={{ height: `${virtualizer.getTotalSize()}px` }}>
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
					{virtualizer.getVirtualItems().map((virtualRow, index) => {
						const row = rows[virtualRow.index];
						return (
							<tr
								onClick={() => onItemSelected(row.original)}
								key={row.id}
								className={`${row.original.error ? "has-error" : ""} ${selectedMessage === row.original ? 'is-selected' : (virtualRow.index % 2 ? 'is-even' : 'is-odd')}`}
								style={{
									height: `${virtualRow.size}px`,
									transform: `translateY(${virtualRow.start - index * virtualRow.size}px)`,
								}}
							>
								{row.getVisibleCells().map(cell => (
									<td key={cell.id}>
										{flexRender(cell.column.columnDef.cell, cell.getContext())}
									</td>
								))}
							</tr>
						);
					})}
				</tbody>
			</table>
		</div>
	);
}
