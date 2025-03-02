import * as React from "react";
import TopPanel from "./TopPanel";
import MessageListWithSidePanel from "./MessageListWithSidePanel";

export default function DevtoolsTab() {
	const topTabHeight = 48;

	return (
		<div style={{ height: '100vh' }}>
			<div style={{ height: `${topTabHeight}px`, padding: '4px' }}>
				<TopPanel />
			</div>
			<div style={{ height: `calc(100% - ${topTabHeight}px)`, overflow: 'auto', }}>
				<MessageListWithSidePanel />
			</div>
		</div>
	);
}
