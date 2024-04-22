// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Colour, EditRole, Mentionable, Role};
use tracing::{error, info};
use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::{accessors, builders};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_ROLES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Edit an existing role.
pub(super) async fn edit(
    ctx: Context<'_>,
    #[description = "Role to edit."] mut role: Role,
    #[description = "Name to set, if any."]
    #[min_length = 1]
    #[max_length = 100]
    name: Option<String>,
    #[description = "Colour for representing, if any. (RRGGBB)"]
    #[min_length = 6]
    #[max_length = 6]
    colour: Option<String>,
    #[description = "Be pinned above lesser roles?"] hoist: Option<bool>,
    #[description = "Be mentionable?"] mentionable: Option<bool>,
) -> Throwable<()> {
    let author = ctx.author();
    let author_name = &author.name;

    let role_name = role.name.clone();
    let role_mention = role.mention();

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = &guild.name;

    let colour = colour
        .map(|hex| u32::from_str_radix(&hex, 16).expect("Failed to parse hex colour"))
        .map(Colour::from);

    let editable_role = EditRole::new()
        .name(&name.unwrap_or(format!("{role_name}")))
        .colour(colour.unwrap_or(Colour::default()))
        .hoist(hoist.is_some())
        .mentionable(mentionable.is_some());

    let result = match role.edit(ctx, editable_role).await {
        Ok(_) => {
            info!("@{author_name} edited @{role_name} in {guild_name}");

            Ok(format!("{role_mention} has been edited."))
        }
        Err(e) => {
            error!("@{author_name} failed to edit @{role_name} in {guild_name}: {e:?}");

            Err(format!("An error occurred while editing {role_mention}."))
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(msg) => builders::replies::build_error_reply_with_embed(msg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
