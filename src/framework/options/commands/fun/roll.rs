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
use tracing::error;

use crate::{utility::components::messages, Context, Error};

#[poise::command(prefix_command, slash_command, category = "Misc", guild_only)]
/// Roll a number of point(s).
pub(crate) async fn roll(
    ctx: Context<'_>,
    #[description = "The number between point(s), if any."]
    #[min = 1]
    number: Option<u32>,
) -> Result<(), Error> {
    let mut rng = StdRng::from_entropy();

    let user_name = &ctx.author().name;

    let number = match number {
        Some(number) => rng.gen_range(1..number),
        None => rng.gen_range(1..100),
    };

    let reply = messages::reply(format!("{user_name} rolled {number} point(s)."), false);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
