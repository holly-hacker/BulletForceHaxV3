import * as React from "react";
import MessageList, { UnpackedDevtoolsMessage } from "./MessageList";
import { useRef, useState } from "react";
import SidePanel from "./SidePanel";

export default function MessageListWithSidePanel(): React.JSX.Element {
	const scrollRef = useRef<HTMLDivElement>(null);
	const [selectedMessage, setMessage] = useState<UnpackedDevtoolsMessage | null>(null);

	const isOpen = selectedMessage !== null;

	return (
		<div style={{ width: '100%', height: '100%', display: 'flex' }}>
			<div ref={scrollRef} style={{ height: '100%', overflow: 'auto', width: isOpen ? '50%' : '100%' }}>
				<MessageList scrollRef={scrollRef} selectedMessage={selectedMessage} onItemSelected={setMessage} />
			</div>
			{isOpen && (
				<div style={{ width: '50%', overflowX: 'scroll' }}>
					<SidePanel selectedMessage={selectedMessage} onClose={() => setMessage(null)} />
				</div>
			)}
		</div>);
};
