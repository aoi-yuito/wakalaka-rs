// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use rand::Rng;
use serenity::all::{CreateEmbedFooter, Mentionable};
use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::builders;

#[poise::command(
    slash_command,
    category = "Fun",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Roll a dice.
pub(crate) async fn roll(
    ctx: Context<'_>,
    #[description = "Number of dice to roll."]
    #[min = 1]
    #[max = 99]
    number: i32,
    #[description = "Number of sides on the dice."]
    #[min = 4]
    #[max = 20]
    sides: i32,
    #[description = "Modifier to add to the roll, if any."]
    #[min = -99]
    #[max = 99]
    modifier: Option<i32>,
) -> Throwable<()> {
    let dices = &[4, 6, 8, 10, 12, 20];
    let modifier = modifier.unwrap_or(0);

    let rolls = fetch_dnd_rolls(number, sides);
    let roll = fetch_dnd_roll(&rolls, modifier);

    let roll_result = fetch_dnd_roll_result(number, sides, modifier);
    let verbose_roll_result = fetch_verbose_dnd_roll_result(&rolls, modifier);

    let author = ctx.author();
    let author_mention = author.mention();

    let result = if dices.contains(&sides) {
        Ok(format!(
            ":game_die: {author_mention} rolled `{roll_result}` and got `{roll}`.",
        ))
    } else {
        Err(format!("`{number}` is not a valid dice size."))
    };

    let reply = match result {
        Ok(msg) => {
            let embed_footer = CreateEmbedFooter::new(verbose_roll_result);
            let embed = builders::embeds::build_embed(Some(format!("{msg}"))).footer(embed_footer);

            builders::replies::build_reply_with_optional_embed("", &Some(embed), false)
        }
        Err(msg) => builders::replies::build_warning_reply_with_embed(msg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}

fn fetch_verbose_dnd_roll_result(rolls: &Vec<i32>, modifier: i32) -> String {
    if modifier > 0 {
        format!("{rolls:?} + {modifier}")
    } else if modifier < 0 {
        format!("{rolls:?} - {modifier}")
    } else {
        format!("{rolls:?}")
    }
}

fn fetch_dnd_roll_result(number: i32, sides: i32, modifier: i32) -> String {
    if modifier > 0 {
        format!("{number}d{sides}+{modifier}")
    } else if modifier < 0 {
        format!("{number}d{sides}-{modifier}")
    } else {
        format!("{number}d{sides}")
    }
}

fn fetch_dnd_roll(rolls: &Vec<i32>, modifier: i32) -> i32 {
    let roll = rolls.iter().sum::<i32>();

    if modifier > 0 {
        roll + modifier
    } else if modifier < 0 {
        roll - modifier
    } else {
        roll
    }
}

fn fetch_dnd_rolls(number: i32, sides: i32) -> Vec<i32> {
    (0..number)
        .map(|_| rand::thread_rng().gen_range(1..=sides))
        .collect::<Vec<_>>()
}
