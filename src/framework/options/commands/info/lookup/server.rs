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

use serenity::all::Guild;

use crate::{
    utility::components::{embeds, replies},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Get information about a server.
pub async fn server(
    ctx: Context<'_>,
    #[description = "The server to get information of."]
    #[rename = "server"]
    guild: Guild,
) -> Result<(), Error> {
    let owner_id = guild.owner_id;
    let owner = owner_id.to_user(ctx).await?;

    let embed = embeds::lookup_server_command_embed(&guild, &owner);

    let reply = replies::reply_embed(embed, true);
    ctx.send(reply).await?;

    Ok(())
}
