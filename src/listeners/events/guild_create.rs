use serenity::client::Context;
use serenity::model::guild::Guild;

pub async fn guild_create(_: Context, guild: Guild, is_new: bool) {
    if is_new {
        println!("[{}] Skraid has been added to guild {}, which has {} members.", guild.id.0, guild.name, guild.member_count);
    } else {
        println!("[{}] Skraid has connected to guild {}, which has {} members.", guild.id.0, guild.name, guild.member_count);
    }
}
