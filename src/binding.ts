import { URL, fileURLToPath } from 'node:url';
import { loadBinding } from '@node-rs/helper';

// eslint-disable-next-line @typescript-eslint/no-empty-interface
interface BackendBinding {
	helloWorld(): string;
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const { helloWorld }: BackendBinding = loadBinding(fileURLToPath(new URL('..', import.meta.url)));

export { helloWorld };
