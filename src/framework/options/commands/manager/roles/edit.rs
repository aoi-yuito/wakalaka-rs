// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Mentionable, Role};
use tracing::{error, info};

use crate::{
    utils::{components, models},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_ROLES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Alter an existing role.
pub(super) async fn edit(
    ctx: Context<'_>,
    #[description = "The role to alter."] mut role: Role,
    #[description = "The new name for the role."]
    #[min_length = 1]
    #[max_length = 100]
    name: Option<String>,
    #[description = "The new colour for the role. (hexadecimal)"]
    #[min = 3]
    #[max = 11]
    colour: Option<String>,
    #[description = "Whether the role should be pinned above lesser roles."] hoist: Option<bool>,
    #[description = "Whether the role should be mentionable."] mentionable: Option<bool>,
) -> Throwable<()> {
    let author = ctx.author();
    let author_name = &author.name;

    let role_name = role.name.clone();
    let role_mention = role.mention();

    let guild = models::guilds::guild(ctx)?;
    let guild_name = &guild.name;

    let role_builder = if let Some(colour) = colour {
        let colour = crate::utils::hex_to_u32(&colour);

        serenity::builder::EditRole::new()
            .name(&name.unwrap_or(format!("{role_name}")))
            .colour(colour)
            .hoist(hoist.is_some())
            .mentionable(mentionable.is_some())
    } else {
        serenity::builder::EditRole::new()
            .name(&name.unwrap_or(format!("{role_name}")))
            .hoist(hoist.is_some())
            .mentionable(mentionable.is_some())
    };

    let result = match role.edit(ctx, role_builder).await {
        Ok(_) => {
            info!("@{author_name} edited @{role_name} in {guild_name}");
            Ok(format!("{role_mention} has been edited."))
        }
        Err(why) => {
            error!("Failed to edit @{role_name} in {guild_name}: {why:?}");
            Err(format!("An error occurred while editing {role_mention}."))
        }
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
