import * as React from "react";

export default function TopPanel({onClear, onDownload}: {onClear: () => void, onDownload: () => void}) {
	return <>
		<button onClick={() => onClear()}>Clear all messages</button>
		<button onClick={() => onDownload()}>Export messages</button>
	</>;
};
