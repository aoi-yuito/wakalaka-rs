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

use poise::CreateReply;

use crate::{utility::embeds, Context, Error};

use super::{AUTHORS, DESCRIPTION, GITHUB_URL, NAME, RUST_VERSION, VERSION};

/// Gets basic information of yours truly.
#[poise::command(prefix_command, slash_command, category = "Info")]
pub(crate) async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let bot = match ctx.http().get_current_user().await {
        Ok(value) => value,
        Err(why) => {
            return Err(format!("Couldn't get bot information: {why:?}").into());
        }
    };
    let bot_avatar_url = bot.avatar_url().unwrap_or(bot.default_avatar_url());

    let constants = [
        NAME,         // 0
        VERSION,      // 1
        AUTHORS,      // 2
        DESCRIPTION,  // 3
        GITHUB_URL,   // 4
        RUST_VERSION, // 5
    ];

    let embed = embeds::info_embed(&bot_avatar_url, constants);

    let reply = CreateReply::default().embed(embed);
    let _ = ctx.send(reply).await;

    Ok(())
}
