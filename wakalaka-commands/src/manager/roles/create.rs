// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Colour, EditRole, Mentionable};

use wakalaka_core::{
    accessors, builders,
    types::{Context, Throwable},
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
/// Create a new role.
pub(super) async fn create(
    ctx: Context<'_>,
    #[description = "Name to give."]
    #[min_length = 1]
    #[max_length = 100]
    name: String,
    #[description = "Colour to give, if any. (RRGGBB)"]
    #[min_length = 6]
    #[max_length = 6]
    colour: Option<String>,
    #[description = "Be pinned above lesser roles?"] hoist: Option<bool>,
    #[description = "Be mentionable?"] mentionable: Option<bool>,
) -> Throwable<()> {
    let author = ctx.author();
    let author_name = &author.name;

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = &guild.name;

    let colour = colour
        .map(|hex| u32::from_str_radix(&hex, 16).expect("Failed to parse hex colour"))
        .map(Colour::from);

    let editable_role = EditRole::new()
        .name(&name)
        .colour(colour.unwrap_or(Colour::default()))
        .hoist(hoist.is_some())
        .mentionable(mentionable.is_some());

    let result = match guild.create_role(ctx, editable_role).await {
        Ok(role) => {
            let role_mention = role.mention();

            tracing::info!("@{author_name} created @{name} in {guild_name}");

            Ok(format!("{role_mention} has been created."))
        }
        Err(e) => {
            tracing::error!("@{author_name} failed to create @{name} in {guild_name}: {e:?}");

            Err(format!("An error occurred while creating `@{name}`."))
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(emsg) => builders::replies::build_error_reply_with_embed(emsg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
