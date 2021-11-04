import { isAbsolute, join } from 'path';
import { fileURLToPath, URL } from 'url';
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

const env = loadEnv(['DISCORD_TOKEN', 'RPC_ENDPOINT', 'PASSPHRASE', 'DATABASE_URL']);

const databaseUrl = isAbsolute(env.DATABASE_URL)
	? env.DATABASE_URL
	: join(fileURLToPath(new URL('..', import.meta.url)), env.DATABASE_URL);

const _ctx = new Context({
	databaseUrl,
	rpcEndpoint: env.RPC_ENDPOINT,
	passphrase: env.PASSPHRASE,
});
