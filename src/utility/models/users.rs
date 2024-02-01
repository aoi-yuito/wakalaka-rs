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
    all::{User, UserId},
    model::ModelError,
};
use tracing::error;

use crate::Context;

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
