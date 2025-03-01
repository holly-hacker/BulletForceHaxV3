import * as React from 'react';
import { createRoot } from 'react-dom/client';
import { StrictMode } from 'react';
import MessageListWithSidePanel from './components/MessageListWithSidePanel';

const rootNode = document.getElementById('root');
if (!rootNode) throw "no root node found";

const root = createRoot(rootNode);

const App = () => {
	return (
		<div style={{ height: '100vh' }}>
			<div style={{ height: '48px', padding: '4px' }}>
				Hello devtools!
			</div>
			<div style={{ height: 'calc(100% - 48px)', overflow: 'auto', }}>
				<MessageListWithSidePanel />
			</div>
		</div>
	);
}

root.render((
	<StrictMode>
		<App></App>
	</StrictMode>
));
