use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::id::GuildId;

pub async fn guild_member_addition(_: Context, guild_id: GuildId, new_member: Member) {
    // TODO: Add support for per-guild enable/disable command
    println!("{} joined guild {}", new_member.user.name, guild_id.0);
}
