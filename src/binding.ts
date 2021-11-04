import { URL, fileURLToPath } from 'node:url';
import { loadBinding } from '@node-rs/helper';

export const { Context } = loadBinding(
	fileURLToPath(new URL('..', import.meta.url)),
) as typeof import('./binding-types');
