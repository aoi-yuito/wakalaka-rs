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

use serenity::all::Mentionable;
use tracing::error;

use crate::{check_restricted_guild_channel, utility::components::messages, Context, Error};

#[poise::command(prefix_command, slash_command, category = "Fun", guild_only)]
/// Flip a coin.
pub async fn flip(ctx: Context<'_>) -> Result<(), Error> {
    let restricted = check_restricted_guild_channel!(ctx);
    if restricted {
        return Ok(());
    }

    let random = rand::random::<bool>();

    let user_mention = ctx.author().mention();

    let reply = if random {
        messages::reply(
            format!("{user_mention} flipped a coin and got **heads**."),
            false,
        )
    } else {
        messages::reply(
            format!("{user_mention} flipped a coin and got **tails**."),
            false,
        )
    };
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
