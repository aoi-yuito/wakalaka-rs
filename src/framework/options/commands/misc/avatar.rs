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

use poise::CreateReply;
use serenity::{all::User, builder::CreateEmbed};

use crate::{Context, Error};

/// Fetches image of users' avatar.
#[poise::command(
    prefix_command,
    slash_command,
    context_menu_command = "User Avatar",
    category = "Miscellaneous",
    guild_only
)]
pub(crate) async fn avatar(
    ctx: Context<'_>,
    #[description = "Mention of user to fetch avatar of."] user: User,
) -> Result<(), Error> {
    let user_name = &user.name;
    let user_avatar_url = user.avatar_url().unwrap_or(user.default_avatar_url());

    let embed = embed(user_name, user_avatar_url);
    let reply = CreateReply {
        content: None,
        embeds: vec![embed],
        ..Default::default()
    };
    let _ = ctx.send(reply).await;

    Ok(())
}

fn embed(name: &String, url: String) -> CreateEmbed {
    CreateEmbed::default()
        .title(format!("{name}'s avatar"))
        .image(url)
}
