import { isAbsolute, join } from 'node:path';
import { fileURLToPath, URL } from 'node:url';
import pino from 'pino';
import { Context } from './binding.js';

type Env<T extends string[]> = Record<T[number], string>;
function loadEnv<T extends string[]>(vars: T): Env<T> {
	const env = {} as Env<T>;
	for (const name of vars) {
		if (!process.env[name]!) {
			throw new Error(`Missing environment variable: ${name}.`);
		}
		env[name as T[number]] = process.env[name]!;
	}
	return env;
}

const env = loadEnv(['RPC_ENDPOINT', 'PASSPHRASE', 'DATABASE_URL']);

const databaseUrl = isAbsolute(env.DATABASE_URL)
	? env.DATABASE_URL
	: join(fileURLToPath(new URL('..', import.meta.url)), env.DATABASE_URL);

const logger = pino();

let ctx;

try {
	ctx = new Context({
		databaseUrl,
		rpcEndpoint: env.RPC_ENDPOINT,
		passphrase: env.PASSPHRASE,
	});
} catch (error) {
	logger.error("Everything didn't work out as expected:");
	throw error;
}

function isError(value: unknown): value is Error & { code: string } {
	return value instanceof Error;
}

try {
	ctx.balance('');
	logger.error("Everything didn't work out as expected:");
	throw new Error("ctx.balance('') didn't raise an error.");
} catch (error) {
	if (isError(error) && error.message === 'User not found' && error.code === 'GenericFailure') {
		logger.info('Everything works as expected!');
		// eslint-disable-next-line unicorn/no-process-exit
		process.exit(0);
	}
	logger.error("Everything didn't work out as expected:");
	throw error;
}
