use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
	#[envconfig(from = "RPC_ENDPOINT")]
	pub rpc_endpoint: String,

	#[envconfig(from = "DISCORD_TOKEN")]
	pub discord_token: String,

	#[envconfig(from = "PASSPHRASE")]
	pub passphrase: String,
}
