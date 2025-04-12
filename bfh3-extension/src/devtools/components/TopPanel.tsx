import * as React from "react";

export default function TopPanel({onClear}: {onClear: () => void}) {
	return <>
		<button onClick={() => onClear()}>Clear all message</button>
	</>;
};
