// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::builder::EditRole;
use tracing::{error, info};

use crate::{
    utils::{self, components, models},
    Context, Error,
};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    required_bot_permissions = "MANAGE_GUILD | SEND_MESSAGES | MANAGE_ROLES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Create a new role.
pub(super) async fn add(
    ctx: Context<'_>,
    #[description = "The name of the role."]
    #[min_length = 1]
    #[max_length = 100]
    name: String,
    #[description = "The colour of the role. (hexadecimal)"]
    #[min = 3]
    #[max = 11]
    colour: Option<String>,
    #[description = "Whether the role should be pinned above lesser roles."] hoist: Option<bool>,
    #[description = "Whether the role should be mentionable."] mentionable: Option<bool>,
) -> Result<(), Error> {
    let author = ctx.author();
    let author_name = &author.name;
    
    let guild = models::guilds::guild(ctx)?;
    let guild_name = &guild.name;

    let role_builder = if let Some(colour) = colour {
        let colour = utils::hex_to_u32(&colour);

        EditRole::new()
            .name(&name)
            .colour(colour)
            .hoist(hoist.is_some())
            .mentionable(mentionable.is_some())
    } else {
        EditRole::new()
            .name(&name)
            .hoist(hoist.is_some())
            .mentionable(mentionable.is_some())
    };

    let result = match guild.create_role(ctx, role_builder).await {
        Ok(_) => {
            info!("@{author_name} created @{name} in {guild_name}");
            Ok(format!("`@{name}` has been created."))
        }
        Err(why) => {
            error!("Failed to create @{name} in {guild_name}: {why:?}");
            Err(format!("An error occurred in creating `@{name}`."))
        }
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
