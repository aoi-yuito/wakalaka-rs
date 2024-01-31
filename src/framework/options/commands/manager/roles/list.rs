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

use std::{cmp::Reverse, fmt::Write};

use serenity::model::Colour;
use tracing::error;

use crate::{
    check_restricted_guild_channel,
    utility::{
        components::{embeds, replies},
        models,
    },
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Get a list of roles in a server.
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let guild = models::guilds::guild(ctx)?;

    let mut roles = models::roles::roles(ctx)?;

    let mut name_field = String::new();
    let mut colour_field = String::new();
    let mut permissions_field = String::new();

    roles.sort_by_key(|role| Reverse(role.position));

    for role in roles.iter_mut() {
        let role_name = &role.name;
        let role_colour = Colour::hex(role.colour);
        let role_permissions = &role.permissions.bits();

        writeln!(name_field, "{role_name}")?;
        writeln!(colour_field, "{role_colour}")?;
        writeln!(permissions_field, "{role_permissions}")?;
    }

    let embed_fields = vec![
        ("Name", name_field, true),
        ("Colour", colour_field, true),
        ("Permissions", permissions_field, true),
    ];

    let embed = embeds::roles_command_embed(&guild, embed_fields);

    let reply = replies::reply_embed(embed, true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
