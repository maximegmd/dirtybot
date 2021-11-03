use diesel::table;

table! {
	users (discord_id) {
			discord_id -> Text,
			deposit_address -> Text,
			amount -> Text,
	}
}
