import * as React from "react";
import { useEffect, useState, useCallback } from "react";
import { MessageFilter } from "./DevtoolsTab";
import { getParameterTypeName } from "./MessageList";

export default function TopPanel({
	onClear,
	onDownload,
	setFilter,
}: {
	onClear: () => void;
	onDownload: () => void;
	setFilter: (filter: MessageFilter) => void;
}) {
	const [errorsOnly, setErrorsOnly] = useState<boolean>(false);
	const [noInternal, setNoInternal] = useState<boolean>(false);
	const [socketFilter, setSocketFilter] = useState<string>("");
	const [headerTextFilter, setHeaderTextFilter] = useState<string>("");
	const [contentTextFilter, setContentTextFilter] = useState<string>("");

	const newFilter = useCallback<MessageFilter>(
		(msg) => {
			// for performance reasons, these checks should be roughly in order of performance cost
			if (errorsOnly && !msg.hasError) return false;
			if (socketFilter && msg.socketType !== socketFilter) return false;
			if (noInternal && msg.messageType?.startsWith("Internal"))
				return false;

			if (headerTextFilter) {
				// TODO: cache json results? this will get really slow
				const found =
					(msg.detail?.indexOf(headerTextFilter) ?? -1) !== -1 ||
					(msg.messageType?.indexOf(headerTextFilter) ?? -1) !== -1 ||
					msg.direction.indexOf(headerTextFilter) !== -1 ||
					(getParameterTypeName(msg)?.indexOf(headerTextFilter) ??
						1) !== -1;
				if (!found) return false;
			}

			// run this last since it's really expensive
			if (contentTextFilter) {
				// TODO: cache json results? this will get really slow
				const found =
					// eslint-disable-next-line @typescript-eslint/no-unnecessary-condition -- `stringify(undef)` returns undef
					(JSON.stringify(msg.rawMessage)?.indexOf(
						contentTextFilter,
					) ?? -1) !== -1 ||
					// eslint-disable-next-line @typescript-eslint/no-unnecessary-condition -- `stringify(undef)` returns undef
					(JSON.stringify(msg.liftedMessage)?.indexOf(
						contentTextFilter,
					) ?? -1) !== -1 ||
					// eslint-disable-next-line @typescript-eslint/no-unnecessary-condition -- `stringify(undef)` returns undef
					(JSON.stringify(msg.interpretedMessage)?.indexOf(
						contentTextFilter,
					) ?? -1) !== -1;
				if (!found) return false;
			}
			return true;
		},
		[
			errorsOnly,
			noInternal,
			socketFilter,
			headerTextFilter,
			contentTextFilter,
		],
	);

	useEffect(() => setFilter(newFilter), [newFilter, setFilter]);

	return (
		<div style={{ padding: "0.5rem" }}>
			<div>
				<b>[Filters]</b>
				{" "}
				<label htmlFor="socketFilter">Socket type</label>{" "}
				<select
					name="socketFilter"
					id="socketFilter"
					onChange={(x) => setSocketFilter(x.target.value)}
				>
					<option value="">All</option>
					<option value="lobby">Lobby</option>
					<option value="game">Game</option>
				</select>
				{" | "}
				<input
					type="checkbox"
					name="errorsOnly"
					id="errorsOnly"
					onChange={(e) => setErrorsOnly(e.target.checked)}
				/>
				<label htmlFor="errorsOnly">Only errors</label>
				{" | "}
				<input
					type="checkbox"
					name="noInternal"
					id="noInternal"
					onChange={(e) => setNoInternal(e.target.checked)}
				/>
				<label htmlFor="noInternal">Hide internal ops</label>
				{" | "}
				<label htmlFor="headerTextFilter">Search table</label>{" "}
				<input
					type="text"
					name="headerTextFilter"
					id="headerTextFilter"
					onChange={(e) => setHeaderTextFilter(e.target.value)}
				/>
				{" | "}
				<label htmlFor="contentTextFilter">
					Search content (slow!)
				</label>{" "}
				<input
					type="text"
					name="contentTextFilter"
					id="contentTextFilter"
					onChange={(e) => setContentTextFilter(e.target.value)}
				/>
			</div>

			<div style={{ padding: "0.25rem" }}>
				<button onClick={() => onClear()}>Clear all</button>
				<button onClick={() => onDownload()}>Export</button>
			</div>
		</div>
	);
}
