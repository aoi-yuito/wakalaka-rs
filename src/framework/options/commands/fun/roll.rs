// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use rand::Rng;
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
    #[description = "The number of dice to roll."]
    #[min = 1]
    #[max = 99]
    number: i32,
    #[description = "The number of sides on the dice."]
    #[min = 4]
    #[max = 20]
    sides: i32,
    #[description = "The modifier to add to the roll, if any."]
    #[min = -99]
    #[max = 99]
    modifier: Option<i32>,
) -> Throwable<()> {
    let d = &[4, 6, 8, 10, 12, 20];
    if !d.contains(&sides) {
        let reply = builders::replies::error_reply_embed(
            format!("{sides} is not a supported dice size!"),
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let modifier = modifier.unwrap_or(0);

    let formatted_roll = if modifier > 0 {
        format!("{number}d{sides}+{modifier}")
    } else if modifier < 0 {
        format!("{number}d{sides}{modifier}")
    } else {
        format!("{number}d{sides}")
    };

    let roll_result = (0..number)
        .map(|_| rand::thread_rng().gen_range(1..=sides))
        .sum::<i32>()
        + modifier;

    let author = ctx.author();
    let author_mention = author.mention();

    let reply = builders::replies::reply_embed(
        format!("{author_mention} rolled **{formatted_roll}** and got `{roll_result}`!",),
        false,
    );

    ctx.send(reply).await?;

    Ok(())
}
