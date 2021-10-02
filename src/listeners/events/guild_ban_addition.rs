use serenity::client::Context;
use serenity::model::id::GuildId;
use serenity::model::user::User;

pub async fn guild_ban_addition(_: Context, guild_id: GuildId, banned_user: User) {
    println!("[{}] Guild has banned {} ({})", guild_id.0, banned_user.name, banned_user.id.0)
}
