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

use crate::{
    database::{restricted_guild_channels, restricted_users},
    Context, Error,
};

pub async fn handle(ctx: Context<'_>) -> Result<bool, Error> {
    let (restricted_user, restricted_guild_channel) = (
        restricted_users::check_restricted_user(ctx).await,
        restricted_guild_channels::check_restricted_guild_channel(ctx).await,
    );
    if restricted_user || restricted_guild_channel {
        Ok(false)
    } else {
        Ok(true)
    }
}
