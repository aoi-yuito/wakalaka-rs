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

use rand::{rngs::StdRng, Rng, SeedableRng};
use serenity::all::Mentionable;

use crate::{utility::components::messages, Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Miscellaneous",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Roll a number of points.
pub async fn roll(
    ctx: Context<'_>,
    #[description = "The number between points, if any."]
    #[min = 1]
    number: Option<u32>,
) -> Result<(), Error> {
    if let Some(number) = number {
        if number < 2 {
            let reply = messages::error_reply(None, "Number must be greater than `1`!", true);
            ctx.send(reply).await?;

            return Ok(());
        }
    }

    let mut rng = StdRng::from_entropy();

    let generated_number = match number {
        Some(number) => rng.gen_range(1..number),
        None => rng.gen_range(1..100),
    };

    let user_mention = ctx.author().mention();

    let reply = if generated_number == 1 {
        messages::reply(None, 
            format!("{user_mention} rolled {generated_number} point!"),
            false,
        )
    } else {
        messages::reply(None, 
            format!("{user_mention} rolled {generated_number} points!"),
            false,
        )
    };
    ctx.send(reply).await?;

    Ok(())
}
