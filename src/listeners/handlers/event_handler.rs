use crate::listeners::events::{guild_member_addition, message, ready};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready, guild::Member, id::GuildId},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member) {
        guild_member_addition::guild_member_addition(ctx, guild_id, new_member).await;
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        message::message(ctx, new_message).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await;
    }
}
