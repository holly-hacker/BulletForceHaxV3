import * as React from 'react';
import { createRoot } from 'react-dom/client';
import { StrictMode } from 'react';
import DevtoolsTab from './components/DevtoolsTab';

const rootNode = document.getElementById('root');
if (!rootNode) throw "no root node found";

const root = createRoot(rootNode);

root.render((
	<StrictMode>
		<DevtoolsTab />
	</StrictMode>
));
