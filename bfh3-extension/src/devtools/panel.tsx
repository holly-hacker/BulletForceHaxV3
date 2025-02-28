import * as React from 'react';
import { createRoot } from 'react-dom/client';
import MessageTest from './components/messageTest';
import { StrictMode, useRef } from 'react';

const rootNode = document.getElementById('root');
if (!rootNode) throw "no root node found";

const root = createRoot(rootNode);

const App = () => {
	const scrollRef = useRef<HTMLDivElement>(null);

	return (
		<div style={{ height: '100vh' }}>
			<div style={{ height: "48px" }}>
				Hello devtools!
			</div>
			<div ref={scrollRef} style={{ height: 'calc(100% - 48px)', overflow: 'auto', }}>
				<MessageTest scrollRef={scrollRef}></MessageTest>
			</div>
		</div>
	);
}

root.render((
	<StrictMode>
		<App></App>
	</StrictMode>));
