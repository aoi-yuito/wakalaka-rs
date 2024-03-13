// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Mentionable;

use crate::{utils::builders, Context, Throwable};

#[poise::command(
    slash_command,
    category = "Miscellaneous",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Roll a dice.
pub(crate) async fn roll(
    ctx: Context<'_>,
    #[description = "The number to roll."]
    #[min = 1]
    #[max = 2147483647] // i32::MAX
    number: Option<i32>,
) -> Throwable<()> {
    let number = number.unwrap_or(100);

    let rolled_number = rand::random::<i32>() % number + 1;

    let author = ctx.author();
    let author_mention = author.mention();

    let reply = if rolled_number == 1 {
        builders::replies::reply_embed(
            format!("{author_mention} rolled {rolled_number} point!"),
            false,
        )
    } else {
        builders::replies::reply_embed(
            format!("{author_mention} rolled {rolled_number} points!"),
            false,
        )
    };

    ctx.send(reply).await?;

    Ok(())
}
