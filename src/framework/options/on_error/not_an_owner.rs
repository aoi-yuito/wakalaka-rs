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

use crate::{
    utility::{components::messages, models},
    Context,
};

pub(crate) async fn handle(ctx: Context<'_>) {
    let owner = models::owner_raw(ctx.serenity_context()).await;
    if let Some(owner) = owner {
        let owner_mention = owner.mention();

        let reply = messages::info_reply(None, 
            format!("Only {owner_mention} can invoke the following command!"),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Failed to send reply: {why:?}");
        }

        return;
    }
}
