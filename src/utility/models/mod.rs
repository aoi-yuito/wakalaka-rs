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

use serenity::all::CurrentApplicationInfo;
use tracing::error;

pub mod channels;
pub mod guilds;
pub mod members;
pub mod roles;
pub mod users;

pub async fn current_application_info_raw(
    ctx: &crate::serenity::Context,
) -> Option<CurrentApplicationInfo> {
    match ctx.http.get_current_application_info().await {
        Ok(value) => Some(value),
        Err(why) => {
            error!("Couldn't get current application info: {why:?}");
            None
        }
    }
}
