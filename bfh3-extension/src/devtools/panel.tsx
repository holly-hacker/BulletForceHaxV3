import * as React from 'react';
import { createRoot } from 'react-dom/client';
import MessageTest from './components/messageTest';

const rootNode = document.getElementById('root');
if (!rootNode) throw "no root node found";

const root = createRoot(rootNode);
root.render((
	<>
		<h1>Hello devtools panel!</h1>
		<MessageTest></MessageTest>
	</>));
