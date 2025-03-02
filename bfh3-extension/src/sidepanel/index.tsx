import * as React from 'react';
import { createRoot } from 'react-dom/client';

const rootNode = document.getElementById('root');
if (!rootNode) throw Error("no root node found");

const root = createRoot(rootNode);
root.render(<h1>Hello from react!</h1>);
