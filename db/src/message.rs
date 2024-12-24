use model::data::*;
use sqlx::MySqlPool;

pub async fn add_message(pool: &MySqlPool, data: Data) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO
            message
            (
                channel_name,
                channel_id,
                author_name,
                author_discriminator,
                author_id,
                author_avatar_url,
                author_bot,
                guild_name,
                guild_id,
                guild_icon_url,
                message_content,
                message_id,
                message_clean_content,
                message_reference_channel_id,
                message_reference_guild_id,
                message_reference_message_id
            )
        VALUES
            (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        data.channel.name,
        data.channel.id,
        data.author.username,
        data.author.discriminator,
        data.author.id,
        data.author.avatar_url,
        data.author.bot,
        data.guild.name,
        data.guild.id,
        data.guild.icon_url,
        data.message.content,
        data.message.id,
        data.message.clean_content,
        data.message.reference.clone().map(|r| r.channel_id),
        data.message.reference.clone().map(|r| r.guild_id),
        data.message.reference.clone().map(|r| r.message_id),
    )
    .execute(pool)
    .await?;
    Ok(())
}
