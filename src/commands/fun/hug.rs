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

use serenity::all::{Mentionable, User};

use crate::{check_channel_restriction, Context, Error};

/// Hugs one of your fellow users.
#[poise::command(slash_command)]
pub(crate) async fn hug(
    ctx: Context<'_>,
    #[description = "Mention of user to firmly hug."] user: User,
) -> Result<(), Error> {
    check_channel_restriction!(ctx);

    let user_mention = ctx.author().mention();
    let other_mention = user.mention();

    let message = format!("{user_mention} :people_hugging: {other_mention}");
    let _ = ctx.reply(message).await;

    Ok(())
}
