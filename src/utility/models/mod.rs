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

pub mod channels;
pub mod guilds;
pub mod members;
pub mod roles;
pub mod users;

use serenity::all::{CurrentApplicationInfo, User, UserId};
use tracing::error;

use crate::{Context, Error};

pub fn author_name(
    ctx: Context<'_>,
) -> Result<&String, Error> {
    let author = author(ctx)?;
    let author_name = &author.name;
    Ok(author_name)
}

pub fn author_id(
    ctx: Context<'_>,
) -> Result<&UserId, Error> {
    let author = author(ctx)?;
    let author_id = &author.id;
    Ok(author_id)
}

pub fn author(ctx: Context<'_>) -> Result<&User, Error> {
    let author = ctx.author();
    Ok(author)
}

pub async fn current_application_name_raw(ctx: &crate::serenity::Context) -> String {
    match current_application_info_raw(ctx).await {
        Some(value) => value.name,
        None => {
            error!("Couldn't get current application name");
            String::new()
        }
    }
}

pub async fn current_application_info_raw(
    ctx: &crate::serenity::Context,
) -> Option<CurrentApplicationInfo> {
    match ctx.http.get_current_application_info().await {
        Ok(value) => Some(value),
        Err(why) => {
            error!("Couldn't get current application info: {why:?}");
            None
        }
    }
}
