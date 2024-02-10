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

use serenity::{
    all::{CurrentUser, Mention, Mentionable, User, UserId},
    model::ModelError,
};
use tracing::error;

use crate::Context;

pub fn author_mention(ctx: Context<'_>) -> Result<Mention, ModelError> {
    let author = author(ctx)?;
    Ok(author.mention())
}

pub fn author_name(ctx: Context<'_>) -> Result<&String, ModelError> {
    let author = author(ctx)?;
    Ok(&author.name)
}

pub fn author_id(ctx: Context<'_>) -> Result<&UserId, ModelError> {
    let author = author(ctx)?;
    Ok(&author.id)
}

pub fn author(ctx: Context<'_>) -> Result<&User, ModelError> {
    Ok(ctx.author())
}

pub async fn user_mention(ctx: Context<'_>, user_id: UserId) -> Result<Mention, ModelError> {
    Ok(user(ctx, user_id).await?.mention())
}

pub async fn user_name(ctx: Context<'_>, user_id: UserId) -> Result<String, ModelError> {
    Ok(user(ctx, user_id).await?.name)
}

pub async fn user(ctx: Context<'_>, user_id: UserId) -> Result<User, ModelError> {
    match user_id.to_user(ctx).await {
        Ok(user) => Ok(user),
        Err(why) => {
            error!("Couldn't get user: {why:?}");
            return Err(ModelError::MemberNotFound);
        }
    }
}

pub async fn bot(ctx: Context<'_>) -> Result<CurrentUser, ModelError> {
    match ctx.http().get_current_user().await {
        Ok(current_user) => Ok(current_user),
        Err(why) => {
            error!("Couldn't get current user: {why:?}");
            return Err(ModelError::MemberNotFound);
        }
    }
}
