// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

use poise::serenity_prelude::Context;
use serenity::all::Guild;
use sqlx::SqlitePool;
use tracing::error;

use crate::Data;

pub(crate) async fn handle(guild: &Guild, is_new: bool, ctx: &Context, data: &Data) {
    if !is_new {
        return;
    }

    let pool = &data.pool;

    let (guild_id, guild_owner_id, guild_preferred_locale) = (
        i64::from(guild.id),
        i64::from(guild.owner_id),
        guild.preferred_locale.clone(),
    );
    let guild_owner_locale = match guild.owner_id.to_user(&ctx.http).await {
        Ok(user) => user.locale,
        Err(why) => {
            error!("Failed to get guild owner's locale: {why:?}");
            return;
        }
    };

    insert_users(guild_owner_id, guild_owner_locale, pool).await;
    insert_guilds(guild_id, guild_owner_id, guild_preferred_locale, pool).await;
}

async fn insert_guilds(id: i64, owner_id: i64, preferred_locale: String, pool: &SqlitePool) {
    let guild_query = sqlx::query(
        "INSERT INTO guilds (id, owner_id, preferred_locale) VALUES (?, ?, ?) ON CONFLICT (id) DO NOTHING",
    ).bind(id).bind(owner_id).bind(preferred_locale);
    if let Err(why) = guild_query.execute(pool).await {
        error!("Couldn't add guild to database: {why:?}");
    }
}

async fn insert_users(id: i64, locale: Option<String>, pool: &SqlitePool) {
    let user_query =
        sqlx::query("INSERT INTO users (id, locale) VALUES (?, ?) ON CONFLICT (id) DO NOTHING")
            .bind(id)
            .bind(locale);
    if let Err(why) = user_query.execute(pool).await {
        error!("Couldn't add user to database: {why:?}");
    }
}
