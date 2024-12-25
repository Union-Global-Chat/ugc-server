use sqlx::MySqlPool;

pub async fn add_token(pool: &MySqlPool, bot_id: i64, token: String) -> anyhow::Result<()> {
    sqlx::query!("INSERT INTO token VALUES (?, ?)", bot_id, token)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn del_token(pool: &MySqlPool, bot_id: i64) -> anyhow::Result<()> {
    sqlx::query!("DELETE FROM token WHERE bot_id = ?", bot_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_token(pool: &MySqlPool, token: String) -> anyhow::Result<Option<i64>> {
    let row = sqlx::query!("SELECT bot_id FROM token WHERE token = ?", token)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|r| r.bot_id))
}