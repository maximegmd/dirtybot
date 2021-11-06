// @types/node and dlopen typings........... :/

import { URL, fileURLToPath } from 'node:url';
import { toNamespacedPath } from 'node:path';
import { constants } from 'node:os';

declare global {
	// eslint-disable-next-line @typescript-eslint/no-namespace, @typescript-eslint/no-unused-vars
	namespace NodeJS {
		export interface Process {
			dlopen(module: { exports: Record<PropertyKey, unknown> }, filename: string, flags: unknown): void;
		}
	}
}

const exports = {} as typeof import('./binding-types');

process.dlopen(
	{ exports },
	toNamespacedPath(fileURLToPath(new URL('../index.node', import.meta.url))),
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	(constants as any).dlopen.RTLD_NOW,
);

exports.setupLogger();

export const { Context } = exports;
